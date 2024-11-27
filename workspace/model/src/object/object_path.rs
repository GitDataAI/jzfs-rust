use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,Debug)]
pub enum ObjectType{
    Dir,
    File,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct ObjectPath{
    pub path: String,
    pub name: String,
    pub object_type: ObjectType,
    pub hash: String,
    pub size: i64,
    pub children: Vec<ObjectPath>,
}