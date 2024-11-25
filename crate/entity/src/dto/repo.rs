use serde::Deserialize;
use crate::uuid::Uuid;

#[derive(Deserialize)]
pub struct RepoCreate{
    pub name: String,
    pub bio: String,
    pub visible: bool,
    pub is_group: bool,
    pub group_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct RepoFork{
    pub name: String,
    pub visible: bool,
    pub bio: String,
    pub is_group: bool,
    pub group_id: Option<Uuid>,
    pub fork_id: Uuid,
}