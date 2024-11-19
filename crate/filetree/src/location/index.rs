use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct FileIndex {
    pub path: String,
    pub file_name: String,
    pub hash: String,
    pub size: u64,
    refs: Vec<u8>
}
impl FileIndex {
    pub fn hash(&mut self){
        self.refs = serde_json::to_string(self).unwrap().into_bytes();
    }
    pub fn as_ref(&self) {
    }
}
impl AsRef<[u8]> for FileIndex {
    fn as_ref(&self) -> &[u8] {
        &self.refs
    }
}