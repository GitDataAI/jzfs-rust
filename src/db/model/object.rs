use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{Decode, Encode};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone)]
pub struct ObjectModule{
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub breach_id: Uuid,
    pub file_tree: Vec<Json<FileTree>>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct FileTree{
    pub id_dir: String,
    pub name: String,
    pub hash: String,
    pub message: String,
    pub commit_at: OffsetDateTime,
    pub children: Option<Vec<Box<FileTree>>>
}
