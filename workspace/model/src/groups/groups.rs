use rbatis::{crud, impl_select};
use rbatis::rbdc::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct GroupModel{
    pub uid: Uuid,
    pub name: String,
    pub bio: String,
    pub avatar: String,
    pub created_at: rbatis::rbdc::timestamp::Timestamp,
    pub updated_at: rbatis::rbdc::timestamp::Timestamp,
}


crud!(GroupModel{}, "groups");
impl_select!(GroupModel{select_by_name(name: String) -> Vec => "where name ~* `_#{name}_`"});