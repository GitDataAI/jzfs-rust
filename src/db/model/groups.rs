use sea_orm::*;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "groups")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: String,
    pub location: Option<String>,
    pub links: Vec<String>,
    pub users: Vec<Uuid>,
    pub topics: Vec<String>,
    pub pinned: Vec<Uuid>,
    pub header: Uuid,
    pub create_to:Uuid,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}