pub mod core_git;
pub mod git_core;

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
    pub path : String,
    pub node : String,
}
