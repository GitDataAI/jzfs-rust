use rbatis::crud;
use serde::{Deserialize, Serialize};
use crate::time::OffsetDateTime;
use crate::uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct UnitEntity{
    pub uid: Uuid,
    pub org_id: Uuid,
    pub team_id: Uuid,
    pub name: String,
    pub users: Vec<Uuid>,
    pub repos: Vec<Uuid>,
    
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
    pub create_by: Uuid
}

impl UnitEntity {
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            org_id: Uuid::new_v4(),
            team_id: Uuid::new_v4(),
            name: "".to_string(),
            users: vec![],
            repos: vec![],
            create_at: OffsetDateTime::now_utc(),
            update_at: OffsetDateTime::now_utc(),
            create_by: Uuid::new_v4(),
        }
    }
}


crud!(UnitEntity{},"units");