use serde::{Deserialize, Serialize};
use rbatis::rbdc::Uuid;
#[derive(Deserialize,Serialize,Clone,Debug,Eq, PartialEq)]
pub enum GroupUsersAccessEnum{
    AccessNone,
    AccessOwner,
    AccessRead(Vec<Uuid>),
    AccessWrite(Vec<Uuid>),
    AccessAdmin(Vec<Uuid>),
}


#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct GroupUsersAccess{
    pub access: GroupUsersAccessEnum,
    pub join_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}