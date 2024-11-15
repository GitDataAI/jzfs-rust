use sqlx::types::Json;
use uuid::Uuid;
use filetree::tree::FileTree;

pub struct Model{
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub name: String,
    pub filetree: Json<FileTree>
}