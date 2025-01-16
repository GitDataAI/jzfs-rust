use std::collections::HashMap;
use std::io;

use actix_files::NamedFile;

use crate::mount::local::LocalStorage;
use crate::mount::nfs::NfsStorage;
use crate::mount::s3::S3Storage;
use crate::rpc::RepositoryStoragePosition;
use crate::service::GitServiceType;

pub mod local;
pub mod nfs;
pub mod s3;

#[derive(Clone)]
pub struct StoragePool {
    pub(crate) s3 : HashMap<String, S3Storage>,
    pub(crate) local : HashMap<String, LocalStorage>,
    pub(crate) nfs : HashMap<String, NfsStorage>,
}

impl Default for StoragePool {
    fn default() -> Self {
        Self::new()
    }
}

impl StoragePool {
    pub fn new() -> Self {
        StoragePool {
            s3 : HashMap::new(),
            local : HashMap::new(),
            nfs : HashMap::new(),
        }
    }

    pub fn add_s3(&mut self, name : String, storage : S3Storage) {
        self.s3.insert(name, storage);
    }

    pub fn add_local(&mut self, name : String, storage : LocalStorage) {
        self.local.insert(name, storage);
    }
    pub fn add_nfs(&mut self, name : String, storage : NfsStorage) {
        self.nfs.insert(name, storage);
    }
    pub fn get(&self, node : RepositoryStoragePosition) -> Option<StorageSingleton> {
        match node {
            RepositoryStoragePosition::S3(x) => {
                if let Some(storage) = self.s3.get(&x.node) {
                    return Some(StorageSingleton::S3(storage.clone()));
                }
            }
            RepositoryStoragePosition::Local(x) => {
                if let Some(storage) = self.local.get(&x.node) {
                    return Some(StorageSingleton::Local(storage.clone()));
                }
            }
            RepositoryStoragePosition::Nfs(x) => {
                if let Some(storage) = self.nfs.get(&x.node) {
                    return Some(StorageSingleton::Nfs(storage.clone()));
                }
            }
        }
        None
    }
}

#[derive(Clone)]
pub enum StorageSingleton {
    S3(S3Storage),
    Local(LocalStorage),
    Nfs(NfsStorage),
}

impl StorageSingleton {
    pub async fn refs(
        &self,
        path : &str,
        service : GitServiceType,
        version : Option<&str>,
    ) -> io::Result<String> {
        match self {
            StorageSingleton::S3(x) => x.refs(path, service, version).await,
            StorageSingleton::Local(x) => x.refs(path, service, version).await,
            StorageSingleton::Nfs(x) => x.refs(path, service, version).await,
        }
    }

    pub async fn text(&self, path : &str, file_path : &str) -> io::Result<NamedFile> {
        match self {
            StorageSingleton::S3(x) => x.text(path, file_path).await,
            StorageSingleton::Local(x) => x.text(path, file_path).await,
            StorageSingleton::Nfs(x) => x.text(path, file_path).await,
        }
    }
}
