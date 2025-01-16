use std::path::PathBuf;

use async_trait::async_trait;
use gitdata::rpc::core_git::*;
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
impl rep_repository_server::RepRepository for CoreGit {
    async fn create(
        &self,
        request : Request<RepositoryStoragePosition>,
    ) -> Result<Response<RepositoryStoragePosition>, Status> {
        let request = request.into_inner();
        let storage = match self.storage.node.get(&request.node) {
            Some(storage) => storage,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        match storage.create_repository(request.path.clone()).await {
            Ok(_) => {
                return Ok(Response::new(RepositoryStoragePosition {
                    node : request.node,
                    path : request.path,
                }));
            }
            Err(e) => {
                return Err(Status::internal(e.to_string()));
            }
        }
    }
    async fn add_file(
        &self,
        request : Request<RepositoryAddFileRequest>,
    ) -> Result<Response<RepositoryAddFilesResponse>, Status> {
        let request = request.into_inner();
        let node = match request.repository_storage_position {
            Some(node) => node,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        let storage = match self.storage.node.get(&node.node) {
            Some(storage) => storage,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        match storage
            .add_file(
                node.path.clone(),
                request.path.clone(),
                request.content.clone(),
                request.email,
                request.user,
                request.message,
                request.file_name,
                request.branch,
            )
            .await
        {
            Ok(_) => {
                return Ok(Response::new(RepositoryAddFilesResponse {
                    code : "200".to_string(),
                }));
            }
            Err(e) => {
                return Err(Status::internal(e.to_string()));
            }
        }
    }
    async fn sync_branch(
        &self,
        request : tonic::Request<RepositoryStoragePosition>,
    ) -> std::result::Result<tonic::Response<RepositorySyncBranchResponse>, tonic::Status> {
        let request = request.into_inner();
        let storage = match self.storage.node.get(&request.node) {
            Some(storage) => storage,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        match storage.branch(request.path.clone()).await {
            Ok(x) => {
                return Ok(Response::new(RepositorySyncBranchResponse { branches : x }));
            }
            Err(e) => {
                return Err(Status::internal(e.to_string()));
            }
        }
    }
    async fn sync_commit(
        &self,
        request : tonic::Request<RepositoryCommitRequest>,
    ) -> std::result::Result<tonic::Response<RepositorySyncCommitResponse>, tonic::Status> {
        let request = request.into_inner();
        let node = match request.position {
            Some(node) => node,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        let storage = match self.storage.node.get(&node.node) {
            Some(storage) => storage,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        match storage.commit(node.path.clone(), request.branch).await {
            Ok(x) => {
                return Ok(Response::new(RepositorySyncCommitResponse { commits : x }));
            }
            Err(e) => {
                return Err(Status::internal(e.to_string()));
            }
        }
    }
    async fn sync_blob(
        &self,
        request : tonic::Request<RepositoryFileRequest>,
    ) -> std::result::Result<tonic::Response<RepositoryFileResponse>, tonic::Status> {
        let request = request.into_inner();
        let node = match request.position {
            Some(node) => node,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        let storage = match self.storage.node.get(&node.node) {
            Some(storage) => storage,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        match storage
            .get_file(
                node.path.clone(),
                request.path.clone(),
                request.hash.clone(),
            )
            .await
        {
            Ok(x) => Ok(Response::new(RepositoryFileResponse {
                hash : request.hash,
                path : request.path.clone(),
                size : x.len().to_string(),
                name : PathBuf::from(request.path)
                    .file_name()
                    .unwrap_or("".as_ref())
                    .to_str()
                    .unwrap_or("")
                    .to_string(),
                mode : "0".to_string(),
                content : x,
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
    async fn sync_tags(
        &self,
        request : tonic::Request<RepositoryStoragePosition>,
    ) -> std::result::Result<tonic::Response<RepositoryTagsResponse>, tonic::Status> {
        let request = request.into_inner();

        let storage = match self.storage.node.get(&request.node) {
            Some(storage) => storage,
            None => {
                return Err(Status::invalid_argument("node not found"));
            }
        };
        match storage.tage(request.path.clone()).await {
            Ok(x) => Ok(Response::new(RepositoryTagsResponse { tags : x })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
