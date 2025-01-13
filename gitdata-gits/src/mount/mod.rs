use std::collections::HashMap;
use std::io;
use std::io::Error;

use actix_files::NamedFile;
use async_trait::async_trait;
use bytes::Bytes;
use futures_core::Stream;

use crate::mount::local::LocalStorage;
use crate::mount::nfs::NfsStorage;
use crate::mount::s3::S3Storage;
use crate::rpc::RepositoryStoragePosition;
use crate::service::GitServiceType;

pub mod local;
pub mod nfs;
pub mod s3;

#[async_trait]
pub trait GitStorage: Send + Sync + 'static {
    async fn refs(
        &self,
        path: &str,
        service: GitServiceType,
        version: Option<&str>,
    ) -> io::Result<String>;
    async fn text(&self, path: &str, file_path: &str) -> io::Result<NamedFile>;
    async fn pack(
        &self,
        path: String,
        service: GitServiceType,
        version: Option<String>,
        gzip: bool,
        payload: Bytes,
    ) -> io::Result<impl Stream<Item = Result<Bytes, Error>>>;
}

pub struct StoragePool {
    s3: HashMap<String, S3Storage>,
    local: HashMap<String, LocalStorage>,
    nfs: HashMap<String, NfsStorage>,
}

impl StoragePool {
    pub fn new() -> Self {
        StoragePool {
            s3: HashMap::new(),
            local: HashMap::new(),
            nfs: HashMap::new(),
        }
    }

    pub fn add_s3(&mut self, name: String, storage: S3Storage) {
        self.s3.insert(name, storage);
    }

    pub fn add_local(&mut self, name: String, storage: LocalStorage) {
        self.local.insert(name, storage);
    }
    pub fn add_nfs(&mut self, name: String, storage: NfsStorage) {
        self.nfs.insert(name, storage);
    }
    pub fn get(&self, node: RepositoryStoragePosition) -> Option<StorageSingleton> {
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
        return None;
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
        path: &str,
        service: GitServiceType,
        version: Option<&str>,
    ) -> io::Result<String> {
        match self {
            StorageSingleton::S3(x) => x.refs(path, service, version).await,
            StorageSingleton::Local(x) => x.refs(path, service, version).await,
            StorageSingleton::Nfs(x) => x.refs(path, service, version).await,
        }
    }

    pub async fn text(&self, path: &str, file_path: &str) -> io::Result<NamedFile> {
        match self {
            StorageSingleton::S3(x) => x.text(path, file_path).await,
            StorageSingleton::Local(x) => x.text(path, file_path).await,
            StorageSingleton::Nfs(x) => x.text(path, file_path).await,
        }
    }
}
