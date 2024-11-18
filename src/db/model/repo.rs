use sea_orm::*;
use sea_orm::prelude::Json;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "repo")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub repo_avatar_url: Option<String>,
    pub origin: Json,
    pub visible: bool,
    pub use_storage: bool,
    pub bio: String,
    pub branch: Vec<Uuid>,
    pub forks: u64,
    pub stars: u64,
    pub fork_from: Option<Uuid>,
    pub create_id: Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug,PartialEq,Eq)]
pub enum RepoOrigin{
    Group(Uuid),
    User(Uuid),
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}