use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRepo{
    pub name: String,
    pub bio: String,
    pub avatar: String,
    pub private: bool,
    pub owner_id: Uuid,
    pub owner_group: bool
}

#[derive(Deserialize)]
pub struct UpdateRepo{
    pub uid: Uuid,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub private: Option<bool>,
}

#[derive(Deserialize)]
pub struct DeleteRepo{
    pub uid: Uuid,
}

#[derive(Deserialize)]
pub struct ListRepo{
    pub uid: Option<Uuid>    
}