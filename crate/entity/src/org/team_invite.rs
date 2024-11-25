use rbatis::crud;
use serde::{Deserialize, Serialize};
use crate::time::OffsetDateTime;
use crate::uuid::Uuid;


#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct TeamUserInvite{
    pub uid: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub create_at: OffsetDateTime,
    pub create_by: Uuid,
    pub active: bool,
    pub allow: bool,
}
impl TeamUserInvite {
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            team_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            create_at: OffsetDateTime::now_utc(),
            create_by: Uuid::new_v4(),
            active: false,
            allow: false,
        }
    }
}

crud!(TeamUserInvite{}, "team_invite");