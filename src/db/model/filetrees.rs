use sea_orm::*;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone,PartialEq,Eq,DeriveEntityModel)]
#[sea_orm(table_name = "filetree")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    /// must use filetree::FileTree
    pub file_tree: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {}