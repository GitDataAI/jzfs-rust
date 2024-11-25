use rbatis::{crud, impl_select};
use jzfs_config::{
    serde::{
        Serialize,
        Deserialize,
    },
    uuid::Uuid,
    time::OffsetDateTime
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserType{
    Pro,
    Unused
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersEntity{
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub hide_email: bool,
    pub passwd: String,
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
    pub nums_stars: Vec<Uuid>,
    pub theme: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

crud!(UsersEntity{},"users");
impl_select!(UsersEntity{select_by_name(name: String) -> Vec => "`where name ~* '_#{name}_'`"});