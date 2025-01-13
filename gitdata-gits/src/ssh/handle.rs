// use crate::mount::{StoragePool, StorageSingleton};
// use crate::rpc::repository::RepositoryRpc;
// use crate::rpc::{RepRepository, RepositoryStoragePosition};
// use crate::service::GitServiceType;
// use async_trait::async_trait;
// use russh::server::{Auth, Handle, Handler, Msg, Session};
// use russh::{Channel, ChannelId, CryptoVec, MethodSet, Pty};
// use russh_keys::PublicKey;
// use std::collections::HashMap;
// use std::process::ExitStatus;
// use tokio::io::AsyncReadExt;
// use tokio::io::{AsyncRead, AsyncWriteExt};
// use tokio::process::ChildStdin;
//
// pub struct GitSshHandle{
//     rpc: RepositoryRpc,
//     stdin: HashMap<ChannelId, ChildStdin>,
//     storage: StoragePool,
// }
//
//
// #[async_trait]
// impl Handler for GitSshHandle {
//     type Error = anyhow::Error;
//
//     async fn auth_none(&mut self, _: &str) -> Result<Auth, Self::Error> {
//         Ok(Auth::Reject { proceed_with_methods: Some(MethodSet::PUBLICKEY), })
//     }
//
//     async fn auth_password(&mut self, _: &str, _: &str) -> Result<Auth, Self::Error> {
//         Ok(Auth::Reject { proceed_with_methods: Some(MethodSet::PUBLICKEY), })
//     }
//
//     async fn auth_publickey(&mut self, user: &str, public_key: &PublicKey) -> Result<Auth, Self::Error> {
//         todo!()
//     }
//
//
//     async fn auth_succeeded(&mut self, session: &mut Session) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     async fn channel_close(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Self::Error> {
//         todo!()
//     }
//
//     async fn channel_eof(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Self::Error> {
//         let stdin = self.stdin.remove(&channel);
//         if let Some(mut stdin) = stdin {
//             stdin.flush().await?;
//             stdin.shutdown().await?;
//         }
//         Ok(())
//     }
//
//     async fn channel_open_session(&mut self, _: Channel<Msg>, _: &mut Session) -> Result<bool, Self::Error> {
//         Ok(true)
//     }
//
//     async fn data(&mut self, channel: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
//         if let Some(stdin) = self.stdin.get_mut(&channel) {
//             stdin.write_all(data).await?;
//         }
//         Ok(())
//     }
//
//     async fn pty_request(&mut self, id: ChannelId, _: &str, _: u32, _: u32, _: u32, _: u32, _: &[(Pty, u32)], session: &mut Session) -> Result<(), Self::Error> {
//         session.request_failure();
//         session.close(id).ok();
//         Ok(())
//     }
//     async fn exec_request(&mut self, channel: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
//         let cmd = match std::str::from_utf8(data){
//             Ok(service_name) => service_name,
//             Err(_) => {
//                 session.request_failure();
//                 session.close(channel).ok();
//                 return Err(
//                     anyhow::anyhow!("Invalid utf8")
//                 );
//             }
//         };
//         fn strip_apostrophes(s: &str) -> &str {
//             if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 && !s[1..s.len() - 1].contains('\'')
//             {
//                 &s[1..s.len() - 1]
//             } else {
//                 s
//             }
//         }
//         let version = match cmd.strip_prefix("version="){
//             Some(version) => {
//                 let version = version.replace("\0", "");
//                 if version.len() < 1 {
//                     "1".to_string()
//                 }else {
//                     version[0..1].to_string()
//                 }
//             },
//             None => "1".to_string(),
//         };
//         let (service,path) = if let Some(rec_pack_path) = cmd.strip_prefix("git-receive-pack ") {
//             (GitServiceType::ReceivePack, strip_apostrophes(rec_pack_path))
//         } else if let Some(upl_ref_path) = cmd.strip_prefix("git-upload-pack ") {
//             (GitServiceType::UploadPack, strip_apostrophes(upl_ref_path))
//         } else if let Some(upl_arc_path) = cmd.strip_prefix("git-upload-archive ") {
//             (GitServiceType::UploadArchive, strip_apostrophes(upl_arc_path))
//         } else {
//             session.request_failure();
//             session.close(channel).ok();
//             return Err(
//                 anyhow::anyhow!(format!("invalid git shell command: {cmd:?}"))
//             );
//         };
//         let (owner, repo) = if let Some((owner, repo)) = path.split_once('/') {
//             (owner.to_string(), repo.to_string())
//         } else {
//             session.request_failure();
//             session.close(channel).ok();
//             return Err(
//                 anyhow::anyhow!(format!("invalid git shell command: {cmd:?}"))
//             );
//         };
//         let path = match self.rpc.path(owner, repo).await{
//             Ok(path) => path,
//             Err(err) => {
//                 session.request_failure();
//                 session.close(channel).ok();
//                 return Err(
//                     anyhow::anyhow!(format!("invalid git shell command: {cmd:?} {err}"))
//                 );
//             }
//         };
//         let storage = match path.clone() {
//             RepositoryStoragePosition::Local(_) => {
//                 match self.storage.get(path.clone()) {
//                     None => {
//                         session.request_failure();
//                         session.close(channel).ok();
//                         return Err(
//                             anyhow::anyhow!("Not Found storage".to_string())
//                         );
//                     }
//                     Some(x) => {
//                         x
//                     }
//                 }
//
//             }
//             RepositoryStoragePosition::S3(_) => {
//                 match self.storage.get(path.clone()) {
//                     None => {
//                         session.request_failure();
//                         session.close(channel).ok();
//                         return Err(
//                             anyhow::anyhow!("Not Found storage".to_string())
//                         );
//                     }
//                     Some(x) => {
//                         x
//                     }
//                 }
//             }
//             RepositoryStoragePosition::Nfs(_) => {
//                 match self.storage.get(path.clone()) {
//                     None => {
//                         session.request_failure();
//                         session.close(channel).ok();
//                         return Err(
//                             anyhow::anyhow!("Not Found storage".to_string())
//                         );
//                     }
//                     Some(x) => {
//                         x
//                     }
//                 }
//             }
//         };
//         let (mut stdout, mut stderr) = match storage {
//             StorageSingleton::Local(x) => {
//                 if let Ok((stdin,stdout)) = x.pack_ssh(path.path().to_string(), service, Some(version)).await {
//                     self.stdin.insert(channel, stdin);
//                     session.channel_success(channel).ok();
//                     stdout
//                 }else {
//                     session.channel_failure(channel).ok();
//                     session.request_failure();
//                     session.close(channel).ok();
//                     return Err(
//                         anyhow::anyhow!("Not Found storage".to_string())
//                     );
//                 }
//             }
//             StorageSingleton::S3(x) => {
//                 if let Ok((stdin,stdout)) = x.pack_ssh(path.path().to_string(), service, Some(version)).await {
//                     self.stdin.insert(channel, stdin);
//                     session.channel_success(channel).ok();
//                     stdout
//                 }else {
//                     session.channel_failure(channel).ok();
//                     session.request_failure();
//                     session.close(channel).ok();
//                     return Err(
//                         anyhow::anyhow!("Not Found storage".to_string())
//                     );
//                 }
//             }
//             StorageSingleton::Nfs(x) => {
//                 if let Ok((stdin,stdout)) = x.pack_ssh(path.path().to_string(), service, Some(version)).await {
//                     self.stdin.insert(channel, stdin);
//                     session.channel_success(channel).ok();
//                     stdout
//                 }else {
//                     session.channel_failure(channel).ok();
//                     session.request_failure();
//                     session.close(channel).ok();
//                     return Err(
//                         anyhow::anyhow!("Not Found storage".to_string())
//                     );
//                 }
//             }
//         };
//         let session_handle = session.handle();
//         let fut = async move {
//             async fn forward<'a, R, Fut, Fwd>(
//                 session_handle: &'a Handle,
//                 chan_id: ChannelId,
//                 r: &mut R,
//                 mut fwd: Fwd,
//             ) -> Result<(), anyhow::Error>
//             where
//                 R: AsyncRead + Send + Unpin,
//                 Fut: std::future::Future<Output = Result<(), CryptoVec>> + 'a,
//                 Fwd: FnMut(&'a Handle, ChannelId, CryptoVec) -> Fut,
//             {
//                 const BUF_SIZE: usize = 1024 * 32;
//
//                 let mut buf = [0u8; BUF_SIZE];
//
//                 loop {
//                     let read = r.read(&mut buf).await?;
//
//                     if read == 0 {
//                         break;
//                     }
//
//                     if fwd(session_handle, chan_id, CryptoVec::from_slice(&buf[..read]))
//                         .await
//                         .is_err()
//                     {
//                         break;
//                     }
//                 }
//
//                 Ok(())
//             }
//
//             use futures::future::FutureExt;
//
//             let stdout_fut = forward(
//                 &session_handle,
//                 channel,
//                 &mut stdout,
//                 |handle, chan, data| async move { handle.data(chan, data).await },
//             )
//                 .fuse();
//
//             tokio::pin!(stdout_fut);
//
//             let stderr_fut = forward(
//                 &session_handle,
//                 channel,
//                 &mut stderr,
//                 |handle, chan, data| async move {
//                     // SSH_EXTENDED_DATA_STDERR = 1
//                     handle.extended_data(chan, 1, data).await
//                 },
//             )
//                 .fuse();
//
//             tokio::pin!(stderr_fut);
//
//             loop {
//                 enum Pipe {
//                     Stdout(Result<(), anyhow::Error>),
//                     Stderr(Result<(), anyhow::Error>),
//                     Exit(std::io::Result<ExitStatus>),
//                 }
//                 let mut shell = match self.stdin.get_mut(&channel) {
//                     Some(x) => x,
//                     None => {
//                         session.request_failure();
//                         session.close(channel).ok();
//                         return Err(
//                             anyhow::anyhow!("Not Found storage".to_string())
//                         );
//                     }
//                 };
//                 let result = tokio::select! {
//                     result = shell.wait() => Pipe::Exit(result),
//                     result = &mut stdout_fut => Pipe::Stdout(result),
//                     result = &mut stderr_fut => Pipe::Stderr(result),
//                 };
//
//                 match result {
//                     Pipe::Stdout(result) => {
//                         let _ = result?;
//                     }
//                     Pipe::Stderr(result) => {
//                         let _ = result?;
//                     }
//                     Pipe::Exit(result) => {
//                         let status = result?;
//
//                         stdout_fut.await?;
//                         stderr_fut.await?;
//
//                         let status_code = status.code().unwrap_or(128) as u32; // TODO: handle signals properly
//
//                         let _ = session_handle
//                             .exit_status_request(channel, status_code)
//                             .await;
//
//                         let _ = session_handle.eof(channel).await;
//                         let _ = session_handle.close(channel).await;
//
//                         break;
//                     }
//                 }
//             }
//
//             Ok::<(), anyhow::Error>(())
//         };
//
//         tokio::spawn(fut);
//         Ok(())
//     }
// }
