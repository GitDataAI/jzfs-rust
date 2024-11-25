use rbatis::{crud, impl_select};
use serde::{Deserialize, Serialize};
use crate::time::OffsetDateTime;
use crate::uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct TeamEntity{
    pub uid: Uuid,
    pub name: String,
    pub bio: String,
    pub repos: Vec<Uuid>,
    pub members: Vec<Uuid>,
    pub nums_repos: usize,
    pub nums_members: usize,
    pub unit: Option<String>,
    pub owner_id: Uuid,
    pub active: bool,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
    pub create_by: Uuid
}
impl TeamEntity{
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            name: "".to_string(),
            bio: "".to_string(),
            repos: vec![],
            members: vec![],
            nums_repos: 0,
            nums_members: 0,
            unit: None,
            owner_id: Uuid::new_v4(),
            active: false,
            create_at: OffsetDateTime::now_utc(),
            update_at: OffsetDateTime::now_utc(),
           create_by: Uuid::new_v4()
        }
    }
}
crud!(TeamEntity{},"teams");
impl_select!(TeamEntity{select_by_name(name: String) -> Vec => "`where name ~* '_#{name}_'`"});