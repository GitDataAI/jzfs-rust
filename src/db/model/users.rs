use sea_orm::*;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: Option<String>,
    pub username: String,
    pub password: String,

    pub avatar_url: Option<String>,
    pub email: String,
    pub bio: Option<String>,
    pub links: Vec<String>,
    pub location: Option<String>,
    pub time_zone: Option<String>,
    pub language: Option<String>,
    pub groups: Vec<Uuid>,

    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}