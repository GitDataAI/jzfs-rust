use rbatis::crud;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::object::object_path::ObjectPath;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct ObjectModel{
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    pub commit_id: Uuid,
    pub path: ObjectPath,
}


crud!(ObjectModel{}, "objects");