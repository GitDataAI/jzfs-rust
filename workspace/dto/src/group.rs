use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GroupCreate{
    pub name: String,
    pub bio: String,
    pub avatar: String
}

#[derive(Deserialize)]
pub struct GroupDelete{
    pub uid: Uuid,
}

#[derive(Deserialize)]
pub struct GroupUpdate{
    pub group_id: Uuid,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>
}

#[derive(Deserialize)]
pub struct GroupOwnerMove{
    pub group_id: Uuid,
    pub new_user_id: Uuid
}

#[derive(Deserialize)]
pub struct GroupAccessUpdate{
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub access: i32,
}

#[derive(Deserialize)]
pub struct GroupInviteUsers{
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub access: i32,
}

#[derive(Deserialize)]
pub struct GroupJoin{
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub allow: bool,
}
#[derive(Deserialize)]
pub struct GroupRemoveUsers{
    pub group_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct GroupCheckNoName{
    pub name: String
}
#[derive(Deserialize)]
pub struct GroupOwnerCheck{
    pub uid: Uuid
}

#[derive(Deserialize)]
pub struct GroupSearchByName{
    pub name: String
}
