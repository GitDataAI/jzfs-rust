use rbatis::crud;
use serde::{Deserialize, Serialize};
use crate::org::team_user_access::TeamUserAccess;
use crate::uuid::Uuid;


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct TeamUserEntity{
    pub uid: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub access: TeamUserAccess
}
impl TeamUserEntity {
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            team_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            access: TeamUserAccess::AccessModeNone
        }
    }
}
crud!(TeamUserEntity{},"team_user");