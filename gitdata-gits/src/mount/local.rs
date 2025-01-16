use std::io;
use std::io::Cursor;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use actix_files::NamedFile;
use async_fn_stream::fn_stream;
use bytes::Bytes;
use flate2::read::GzDecoder;
use futures_core::Stream;
use git2::Signature;

use crate::service::GitServiceType;

#[derive(Clone)]
pub struct LocalStorage {
    pub root : PathBuf,
}

impl LocalStorage {
    pub(crate) async fn refs(
        &self,
        path : &str,
        service : GitServiceType,
        version : Option<&str>,
    ) -> io::Result<String> {
        let mut cmd = Command::new("git");
        cmd.arg(service.to_string());
        cmd.arg("--stateless-rpc");
        cmd.arg("--advertise-refs");
        cmd.arg(".");
        cmd.current_dir(self.root.join(path));
        if !version.is_some() {
            cmd.env("GIT_PROTOCOL", version.unwrap_or(""));
        }
        let output = match cmd.output() {
            Ok(output) => output,
            Err(e) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
        };
        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Error running command",
            ));
        }
        Ok(String::from_utf8(output.stdout).unwrap_or("".to_string()))
    }
    pub(crate) async fn text(&self, path : &str, file_path : &str) -> io::Result<NamedFile> {
        let file_path = self.root.join(path).join(file_path);
        if !file_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
        if file_path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::Other, "File is a directory"));
        }
        Ok(NamedFile::open(file_path)?)
    }
    pub(crate) async fn pack(
        &self,
        path : String,
        service : GitServiceType,
        version : Option<String>,
        gzip : bool,
        payload : Bytes,
    ) -> io::Result<impl Stream<Item = Result<Bytes, Error>> + use<>> {
        let mut cmd = Command::new("git");
        cmd.arg(service.to_string());
        // cmd.arg("receive-pack");
        cmd.arg("--stateless-rpc");
        cmd.arg(".");
        if !version.is_some() {
            cmd.env("GIT_PROTOCOL", version.unwrap_or("".to_string()));
        }
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.current_dir(self.root.join(path));
        let mut git_process = match cmd.spawn() {
            Ok(process) => process,
            Err(e) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
        };
        let mut stdin = git_process.stdin.take().unwrap();
        let mut stdout = git_process.stdout.take().unwrap();
        let bytes = if gzip {
            let mut decoder = GzDecoder::new(Cursor::new(payload));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
            decoded_data
        } else {
            payload.to_vec()
        };
        if let Err(e) = stdin.write_all(&bytes) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error running command {}", e),
            ));
        }
        drop(stdin);
        Ok(fn_stream(move |emitter| async move {
            let mut buffer = [0; 8192];
            loop {
                match stdout.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        emitter
                            .emit(Ok(Bytes::copy_from_slice(&buffer[0..n])))
                            .await;
                    }
                    Err(e) => {
                        emitter.emit(Err(Error::new(io::ErrorKind::Other, e))).await;
                        break;
                    }
                }
            }
        }))
    }
    pub async fn pack_ssh(
        &self,
        path : String,
        service : GitServiceType,
        version : Option<String>,
    ) -> io::Result<(
        tokio::process::ChildStdin,
        (tokio::process::ChildStdout, tokio::process::ChildStderr),
    )> {
        let mut cmd = tokio::process::Command::new("git");
        cmd.arg(service.to_string());
        cmd.arg(".");
        if !version.is_some() {
            cmd.env("GIT_PROTOCOL", version.unwrap_or("".to_string()));
        }
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.current_dir(self.root.join(path));
        let mut git_process = match cmd.spawn() {
            Ok(process) => process,
            Err(e) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
        };
        Ok((
            git_process.stdin.take().unwrap(),
            (
                git_process.stdout.take().unwrap(),
                git_process.stderr.take().unwrap(),
            ),
        ))
    }
    pub(crate) async fn create_repository(&self, path : String) -> anyhow::Result<()> {
        if std::fs::read_dir(&self.root.join(path.clone())).is_ok() {
            return Err(anyhow::anyhow!("Repository Path already exists"));
        }
        let git = git2::Repository::init_bare(&self.root.join(path.clone()));
        if git.is_ok() {
            Ok(())
        } else if let Err(r) = git {
            Err(anyhow::anyhow!("{}", r))
        } else {
            Err(anyhow::anyhow!("Unknown Error"))
        }
    }
    pub(crate) async fn add_file(
        &self,
        path : String,
        file_path : String,
        bytes : Vec<u8>,
        commit_email : String,
        commit_users : String,
        commit_message : String,
        file_name : String,
        branch_name : String,
    ) -> anyhow::Result<()> {
        use anyhow::Context;

        let path = self.root.join(path);
        let tmp = tempfile::tempdir().context("Failed to create temporary directory")?;
        let clone_repository = git2::Repository::clone(path.to_str().unwrap(), tmp.path())
            .context("Failed to clone repository")?;

        let branch = match clone_repository.find_branch(&branch_name, git2::BranchType::Local) {
            Ok(branch) => branch,
            Err(_) => {
                let head_commit = clone_repository
                    .head()
                    .context("Failed to get HEAD")?
                    .peel_to_commit()
                    .context("Failed to peel HEAD to commit")?;
                clone_repository
                    .branch(&branch_name, &head_commit, false)
                    .context("Failed to create branch")?;
                clone_repository
                    .find_branch(&branch_name, git2::BranchType::Local)
                    .context("Failed to find branch after creation")?
            }
        };

        let branch_name = branch
            .name()
            .transpose()
            .context("Failed to get branch name")?
            .map_err(|_| anyhow::anyhow!("Branch name is empty"))?;

        if !branch.is_head() {
            clone_repository
                .set_head(&branch_name)
                .context("Failed to set HEAD to branch")?;
            clone_repository
                .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
                .context("Failed to check out HEAD")?;
        }

        let full_file_path = tmp.path().join(&file_path).join(&file_name);
        std::fs::create_dir_all(
            full_file_path
                .parent()
                .context("Failed to get parent directory")?,
        )?;
        std::fs::write(&full_file_path, bytes).context("Failed to write file")?;

        let relative_path = full_file_path
            .strip_prefix(tmp.path())
            .context("Failed to strip prefix from file path")?;
        let mut index = clone_repository
            .index()
            .context("Failed to get repository index")?;
        index
            .add_path(relative_path)
            .context("Failed to add file to index")?;
        index.write().context("Failed to write index")?;

        let time = chrono::Utc::now().timestamp();
        let time = git2::Time::new(time, 0);
        let user = Signature::new(&commit_users, &commit_email, &time)
            .context("Failed to create commit signature")?;
        let tree = clone_repository
            .find_tree(index.write_tree().context("Failed to write tree")?)
            .context("Failed to find tree")?;
        let parent_commit = clone_repository
            .find_commit(
                branch
                    .get()
                    .target()
                    .context("Failed to get branch target")?,
            )
            .context("Failed to find parent commit")?;
        clone_repository
            .commit(Some("HEAD"), &user, &user, &commit_message, &tree, &[
                &parent_commit,
            ])
            .context("Failed to create commit")?;

        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        clone_repository
            .find_remote("origin")
            .context("Failed to find remote 'origin'")?
            .push(&[refspec.as_str()], Some(&mut git2::PushOptions::new()))
            .context("Failed to push changes to remote")?;

        Ok(())
    }
}
