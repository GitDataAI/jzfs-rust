use std::sync::Arc;

use async_trait::async_trait;
use futures::io;
use gitdata::rpc::git;
use tokio::sync::Mutex;

use crate::rpc::{NodePath, RepRepository, RepositoryAccess, RepositoryStoragePosition};

pub struct RepositoryRpc {
    client: Arc<Mutex<git::rep_repository_client::RepRepositoryClient<tonic::transport::Channel>>>,
}

#[async_trait]
impl RepRepository for RepositoryRpc {
    async fn path(&self, owner: String, repo: String) -> io::Result<RepositoryStoragePosition> {
        let mut client = self.client.lock().await;
        let result = match client.path(git::PathRequest { owner, repo }).await {
            Ok(x) => x.into_inner(),
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }
        };
        match result.r#type {
            0 => Ok(RepositoryStoragePosition::Local(NodePath {
                path: result.path,
                node: result.node,
            })),
            1 => Ok(RepositoryStoragePosition::S3(NodePath {
                path: result.path,
                node: result.node,
            })),
            2 => Ok(RepositoryStoragePosition::Nfs(NodePath {
                path: result.path,
                node: result.node,
            })),
            _ => Err(io::Error::new(io::ErrorKind::Other, "unknown type")),
        }
    }

    async fn token(
        &self,
        owner: String,
        repo: String,
        token: String,
    ) -> io::Result<RepositoryAccess> {
        let mut client = self.client.lock().await;
        match client.token(git::TokenRequest { owner, repo, token }).await {
            Ok(x) => match x.into_inner().access {
                0 => Ok(RepositoryAccess::None),
                1 => Ok(RepositoryAccess::Read),
                2 => Ok(RepositoryAccess::Write),
                3 => Ok(RepositoryAccess::Admin),
                _ => Err(io::Error::new(io::ErrorKind::Other, "unknown type")),
            },
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    async fn publickey(
        &self,
        owner: String,
        repo: String,
        publickey: String,
    ) -> io::Result<RepositoryAccess> {
        let mut client = self.client.lock().await;
        match client
            .publickey(git::PublickeyRequest {
                owner,
                repo,
                publickey,
            })
            .await
        {
            Ok(x) => match x.into_inner().access {
                0 => Ok(RepositoryAccess::None),
                1 => Ok(RepositoryAccess::Read),
                2 => Ok(RepositoryAccess::Write),
                3 => Ok(RepositoryAccess::Admin),
                _ => Err(io::Error::new(io::ErrorKind::Other, "unknown type")),
            },
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
}

impl RepositoryRpc {
    pub async fn new(addr: String) -> io::Result<Self> {
        let channel = tonic::transport::Channel::from_shared(addr)
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?
            .connect()
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;
        Ok(Self {
            client: Arc::new(Mutex::new(
                git::rep_repository_client::RepRepositoryClient::new(channel),
            )),
        })
    }
}
