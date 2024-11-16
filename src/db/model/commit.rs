use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "commit")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub hash: String,
    pub author: String,
    pub message: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    pub file_tree: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}