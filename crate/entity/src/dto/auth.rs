use serde::{Deserialize, Serialize};
use crate::time::OffsetDateTime;
use crate::users::users::{UserType, UsersEntity};
use crate::uuid::Uuid;

#[derive(Deserialize,Serialize)]
pub struct AuthUserNamePassword {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize,Serialize)]
pub struct AuthEmailPassword{
    pub email: String,
    pub password: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthRespDto{
    pub uid: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub hide_email: bool,
    pub user_type: UserType,
    pub local: String,
    pub websites: String,
    pub rands: String,
    pub language: String,
    pub bio: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub max_repo_count: i32,
    pub avatar_url: String,
    pub nums_flowers: Vec<Uuid>,
    pub nums_flowings: Vec<Uuid>,
    pub theme: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}


impl From<&UsersEntity> for AuthRespDto {
    fn from(users: &UsersEntity) -> Self {
        Self{
            uid: users.uid,
            username: users.username.clone(),
            name: users.name.clone(),
            email: users.email.clone(),
            hide_email: users.hide_email,
            user_type: users.user_type.clone(),
            local: users.local.clone(),
            websites: users.websites.clone(),
            rands: users.rands.clone(),
            language: users.language.clone(),
            bio: users.bio.clone(),
            is_active: users.is_active,
            is_admin: users.is_admin,
            max_repo_count: users.max_repo_count,
            avatar_url: users.avatar_url.clone(),
            nums_flowers: users.nums_flowers.clone(),
            nums_flowings: users.nums_flowings.clone(),
            theme: users.theme.clone(),
            created_at: users.created_at,
            updated_at: users.updated_at,
        }
    }
}
