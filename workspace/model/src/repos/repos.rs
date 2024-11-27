use rbatis::{crud, impl_select};
use rbatis::rbdc::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct ReposModel{
    pub uid: Uuid,
    pub name: String,
    pub bio: String,
    pub avatar: String,
    pub owner_id: Uuid,
    pub owner_group: bool,
    pub private: bool,
    pub fork: bool,
    pub fork_from: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

crud!(ReposModel{}, "repos");
impl_select!(ReposModel{select_by_name(name: String, offset: u64, limit: u64) -> Vec =>"where name ~* `_#{name}_` limit #{limit} offset #{offset}"});
