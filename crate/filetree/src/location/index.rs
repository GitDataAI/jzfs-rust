#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct FileIndex {
    pub path: PathBuf,
    pub file_name: String,
    pub hash: String,
    pub size: u64,
    pub offset: u64
}
