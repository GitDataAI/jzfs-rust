use std::collections::HashMap;
use rbatis::crud;
use rbatis::rbdc::JsonV;
use serde::{Deserialize, Serialize};
use rbatis::rbdc::Uuid;
use crate::groups::group_users_access::GroupUsersAccess;



#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct GroupUserModel{
    pub uid: Uuid,
    pub group_id: Uuid,
    pub users_id: JsonV<HashMap<Uuid, GroupUsersAccess>>,
    pub created_at: rbatis::rbdc::timestamp::Timestamp,
    pub updated_at: rbatis::rbdc::timestamp::Timestamp
}

crud!(GroupUserModel{}, "group_users");