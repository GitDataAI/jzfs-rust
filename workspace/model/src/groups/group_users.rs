use std::collections::HashMap;
use rbatis::crud;
use rbatis::rbdc::JsonV;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::groups::group_users_access::GroupUsersAccess;



#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct GroupUserModel{
    pub uid: Uuid,
    pub group_id: Uuid,
    pub users_id: JsonV<HashMap<Uuid, GroupUsersAccess>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}

crud!(GroupUserModel{}, "group_users");