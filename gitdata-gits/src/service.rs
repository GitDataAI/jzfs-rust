pub enum GitServiceType {
    ReceivePack,
    UploadPack,
    UploadArchive,
}

impl GitServiceType {
    pub fn to_string(&self) -> String {
        match self {
            GitServiceType::ReceivePack => "receive-pack".to_string(),
            GitServiceType::UploadPack => "upload-pack".to_string(),
            GitServiceType::UploadArchive => "upload-archive".to_string(),
        }
    }
    pub fn from_string(s : &str) -> Option<GitServiceType> {
        if s.is_empty() {
            return None;
        }
        match s.replace("git-", "").as_ref() {
            "receive-pack" => Some(GitServiceType::ReceivePack),
            "upload-pack" => Some(GitServiceType::UploadPack),
            "upload-archive" => Some(GitServiceType::UploadArchive),
            _ => None,
        }
    }
}
