use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "stars")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub owner_id: Uuid,
    pub stars_repo: Vec<Uuid>,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}