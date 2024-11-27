use std::collections::HashMap;
use rbatis::crud;
use rbatis::rbdc::{JsonV, Uuid};
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct UsersModel{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub passwd: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub pro: bool,
    pub active: bool,
    pub repo: JsonV<HashMap<String, Uuid>>,
    pub created_at: rbatis::rbdc::timestamp::Timestamp,
    pub updated_at: rbatis::rbdc::timestamp::Timestamp,
}


crud!(UsersModel{}, "users");