pub mod core_git;
pub mod git_core;

pub enum RepositoryAccess {
    None,
    Read,
    Write,
    Admin,
}

#[derive(Clone, Debug)]
pub struct NodePath {
    pub path : String,
    pub node : String,
}
