use serde::Deserialize;
use crate::uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTeamReq{
    pub name: String,
    pub bio: String,
}

#[derive(Deserialize)]
pub struct TeamUpdateReq{
    pub org_id: Uuid,
    pub name: Option<String>,
    pub bio: Option<String>,
}
#[derive(Deserialize)]
pub struct TeamDeleteReq{
    pub org_id: Uuid,
    pub apply_id: Uuid
}

#[derive(Deserialize)]
pub struct TeamInviteUsers{
    pub user_email: String,
    pub team_id: Uuid,
}