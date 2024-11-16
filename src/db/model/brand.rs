use sea_orm::*;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "brands")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub name: String,
    pub bio: String,
    pub create_id: String,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}
