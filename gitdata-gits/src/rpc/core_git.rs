use async_trait::async_trait;
use gitdata::rpc;
use gitdata::rpc::core_git::RepositoryCreate;
use gitdata::rpc::core_git::RepositoryStoragePosition;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use crate::mount::StoragePool;

pub struct CoreGit {
    pub storage : StoragePool,
}

impl CoreGit {
    pub fn new(storage : StoragePool) -> Self {
        CoreGit { storage }
    }
}

#[async_trait]
impl rpc::core_git::rep_repository_server::RepRepository for CoreGit {
    async fn create(
        &self,
        request : Request<RepositoryCreate>,
    ) -> Result<Response<RepositoryStoragePosition>, Status> {
        todo!()
    }
}
