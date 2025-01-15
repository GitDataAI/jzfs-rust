use std::sync::Arc;

use futures::io;
use gitdata::rpc::git_core;
use gitdata::rpc::git_core::Repository;
use tokio::sync::Mutex;

use crate::rpc::NodePath;
use crate::rpc::RepositoryStoragePosition;

#[derive(Clone)]
pub struct RepositoryRpc {
    client :
        Arc<Mutex<git_core::rep_repository_client::RepRepositoryClient<tonic::transport::Channel>>>,
}

impl RepositoryRpc {
    pub async fn path(
        &self,
        owner : String,
        repo : String,
    ) -> io::Result<RepositoryStoragePosition> {
        let mut client = self.client.lock().await;
        let result = match client.path(git_core::PathRequest { owner, repo }).await {
            Ok(x) => x.into_inner(),
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }
        };
        match result.r#type {
            0 => Ok(RepositoryStoragePosition::Local(NodePath {
                path : result.path,
                node : result.node,
            })),
            1 => Ok(RepositoryStoragePosition::S3(NodePath {
                path : result.path,
                node : result.node,
            })),
            2 => Ok(RepositoryStoragePosition::Nfs(NodePath {
                path : result.path,
                node : result.node,
            })),
            _ => Err(io::Error::new(io::ErrorKind::Other, "unknown type")),
        }
    }

    pub async fn token(
        &self,
        owner : String,
        repo : String,
        token : String,
    ) -> io::Result<Vec<Repository>> {
        let mut client = self.client.lock().await;
        match client
            .token(git_core::TokenRequest { owner, repo, token })
            .await
        {
            Ok(x) => Ok(x.into_inner().repositories),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    pub async fn publickey(
        &self,
        owner : String,
        repo : String,
        publickey : String,
    ) -> io::Result<Vec<Repository>> {
        let mut client = self.client.lock().await;
        match client
            .publickey(git_core::PublickeyRequest {
                owner,
                repo,
                publickey,
            })
            .await
        {
            Ok(x) => Ok(x.into_inner().repositories),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
}

impl RepositoryRpc {
    pub async fn new(addr : String) -> io::Result<Self> {
        let channel = tonic::transport::Channel::from_shared(addr)
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .connect()
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        Ok(Self {
            client : Arc::new(Mutex::new(
                git_core::rep_repository_client::RepRepositoryClient::new(channel),
            )),
        })
    }
}
