use crate::location::index::FileIndex;
use crate::AsyncFolder;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct FolderIndex {
    pub path: PathBuf,
    pub name: String,
    pub sub_folders: HashMap<String, FolderIndex>,
    pub files: Vec<FileIndex>,
    pub hash: String,
    pub size: u64,
    pub is_root: bool,
}


impl AsyncFolder for FolderIndex {
    type Error = io::Error;

    async fn mkdir(&mut self, local: &str) -> Result<(), Self::Error> {
        todo!()
    }

    async fn rmdir(&mut self, local: &str) -> Result<(), Self::Error> {
        todo!()
    }

    async fn move_files(&mut self, local: &str, remote: &str) -> Result<(), Self::Error> {
        todo!()
    }
}