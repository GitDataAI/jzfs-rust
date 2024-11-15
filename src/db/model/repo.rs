use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use sqlx::types::Json;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "repo")]
pub struct RepoModule{
    pub uid: String,
    pub repo_avatar_url: Option<String>,
    pub origin: Json<RepoOrigin>,
    pub visible: bool,
    pub use_public_storage: bool,
    pub bio: String,
    pub branch: Vec<Uuid>,
    pub forks: u64,
    pub stars: u64,
    pub create_id: Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug,PartialEq,Eq)]
pub enum RepoOrigin{
    Group(Uuid),
    User(Uuid),
}
