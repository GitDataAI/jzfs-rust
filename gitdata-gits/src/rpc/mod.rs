pub mod repository;
use std::io;

use async_trait::async_trait;

#[async_trait]
pub trait RepRepository: Send + Sync + 'static {
    async fn path(&self, owner: String, repo: String) -> io::Result<RepositoryStoragePosition>;
    async fn token(
        &self,
        owner: String,
        repo: String,
        token: String,
    ) -> io::Result<RepositoryAccess>;
    async fn publickey(
        &self,
        owner: String,
        repo: String,
        publickey: String,
    ) -> io::Result<RepositoryAccess>;
}

pub enum RepositoryAccess {
    None,
    Read,
    Write,
    Admin,
}

#[derive(Clone)]
pub enum RepositoryStoragePosition {
    Local(NodePath),
    S3(NodePath),
    Nfs(NodePath),
}
impl RepositoryStoragePosition {
    pub fn path(&self) -> &str {
        match self {
            RepositoryStoragePosition::Local(p) => p.path.as_str(),
            RepositoryStoragePosition::S3(p) => p.path.as_str(),
            RepositoryStoragePosition::Nfs(p) => p.path.as_str(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NodePath {
    pub path: String,
    pub node: String,
}
