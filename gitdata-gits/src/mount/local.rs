use std::io;
use std::io::{Cursor, Error, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use actix_files::NamedFile;
use async_fn_stream::fn_stream;
use bytes::Bytes;
use flate2::read::GzDecoder;
use futures_core::Stream;

use crate::service::GitServiceType;

#[derive(Clone)]
pub struct LocalStorage {
    pub root: PathBuf,
}

impl LocalStorage {
    pub(crate) async fn refs(
        &self,
        path: &str,
        service: GitServiceType,
        version: Option<&str>,
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
    pub(crate) async fn text(&self, path: &str, file_path: &str) -> io::Result<NamedFile> {
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
        path: String,
        service: GitServiceType,
        version: Option<String>,
        gzip: bool,
        payload: Bytes,
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
                return Err(io::Error::new(
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
        path: String,
        service: GitServiceType,
        version: Option<String>,
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
}
