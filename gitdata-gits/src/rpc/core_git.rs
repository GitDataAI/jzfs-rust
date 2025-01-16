use async_trait::async_trait;
use gitdata::rpc;
use gitdata::rpc::core_git::RepositoryAddFileRequest;
use gitdata::rpc::core_git::RepositoryAddFilesResponse;
use gitdata::rpc::core_git::RepositoryCreate;
use gitdata::rpc::core_git::RepositoryStoragePosition as Pos;
use tonic::Request;
use tonic::Response;
use tonic::Status;

use crate::mount::StoragePool;
use crate::mount::StorageSingleton;
use crate::rpc::NodePath;

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
    async fn create(&self, request : Request<RepositoryCreate>) -> Result<Response<Pos>, Status> {
        let request = request.into_inner();
        let storage_position = match request.storage_position {
            Some(storage_position) => storage_position,
            None => {
                return Err(Status::invalid_argument("storage_position is required"));
            }
        };
        let node = NodePath {
            path : storage_position.path.clone(),
            node : storage_position.node.clone(),
        };
        let storge = match storage_position.r#type {
            0 => StorageSingleton::S3(self.storage.s3.get(&node.node).unwrap().clone()),
            1 => StorageSingleton::Local(self.storage.local.get(&node.node).unwrap().clone()),
            2 => StorageSingleton::Nfs(self.storage.nfs.get(&node.node).unwrap().clone()),
            _ => {
                return Err(Status::invalid_argument(
                    "storage_position.r#type is invalid",
                ));
            }
        };
        match storge {
            StorageSingleton::S3(x) => {
                return match x.create_repository(storage_position.path.clone()).await {
                    Ok(_) => Ok(Response::new(Pos {
                        r#type : storage_position.r#type,
                        path : storage_position.path.clone(),
                        node : storage_position.node.clone(),
                    })),
                    Err(e) => Err(Status::internal(e.to_string())),
                };
            }
            StorageSingleton::Local(x) => {
                return match x.create_repository(storage_position.path.clone()).await {
                    Ok(_) => Ok(Response::new(Pos {
                        r#type : storage_position.r#type,
                        path : storage_position.path.clone(),
                        node : storage_position.node.clone(),
                    })),
                    Err(e) => Err(Status::internal(e.to_string())),
                };
            }
            StorageSingleton::Nfs(x) => {
                return match x.create_repository(storage_position.path.clone()).await {
                    Ok(_) => Ok(Response::new(Pos {
                        r#type : storage_position.r#type,
                        path : storage_position.path.clone(),
                        node : storage_position.node.clone(),
                    })),
                    Err(e) => Err(Status::internal(e.to_string())),
                };
            }
        }
    }
    async fn add_file(
        &self,
        request : Request<RepositoryAddFileRequest>,
    ) -> Result<Response<RepositoryAddFilesResponse>, Status> {
        let request = request.into_inner();
        let storage_position = match request.repository_storage_position {
            Some(storage_position) => storage_position,
            None => {
                return Err(Status::invalid_argument("storage_position is required"));
            }
        };
        let node = NodePath {
            path : storage_position.path.clone(),
            node : storage_position.node.clone(),
        };
        let storge = match storage_position.r#type {
            0 => StorageSingleton::S3(self.storage.s3.get(&node.node).unwrap().clone()),
            1 => StorageSingleton::Local(self.storage.local.get(&node.node).unwrap().clone()),
            2 => StorageSingleton::Nfs(self.storage.nfs.get(&node.node).unwrap().clone()),
            _ => {
                return Err(Status::invalid_argument(
                    "storage_position.r#type is invalid",
                ));
            }
        };
        match storge {
            StorageSingleton::S3(x) => {
                match x
                    .add_file(
                        storage_position.path.clone(),
                        request.path.clone(),
                        request.content.clone(),
                        request.email.clone(),
                        request.user.clone(),
                        request.message.clone(),
                        request.file_name.clone(),
                        request.branch.clone(),
                    )
                    .await
                {
                    Ok(_) => Ok(Response::new(RepositoryAddFilesResponse::default())),
                    Err(x) => {
                        return Err(Status::internal(x.to_string()));
                    }
                }
            }
            StorageSingleton::Local(x) => {
                match x
                    .add_file(
                        storage_position.path.clone(),
                        request.path.clone(),
                        request.content.clone(),
                        request.email.clone(),
                        request.user.clone(),
                        request.message.clone(),
                        request.file_name.clone(),
                        request.branch.clone(),
                    )
                    .await
                {
                    Ok(_) => Ok(Response::new(RepositoryAddFilesResponse::default())),
                    Err(x) => return Err(Status::internal(x.to_string())),
                }
            }
            StorageSingleton::Nfs(x) => {
                match x
                    .add_file(
                        storage_position.path.clone(),
                        request.path.clone(),
                        request.content.clone(),
                        request.email.clone(),
                        request.user.clone(),
                        request.message.clone(),
                        request.file_name.clone(),
                        request.branch.clone(),
                    )
                    .await
                {
                    Ok(_) => Ok(Response::new(RepositoryAddFilesResponse::default())),
                    Err(x) => return Err(Status::internal(x.to_string())),
                }
            }
        }
    }
}
