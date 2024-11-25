use jzfs_entity::config::anyhow;
use jzfs_entity::rbatis::rbdc::db::ExecResult;
use jzfs_entity::time::OffsetDateTime;
use jzfs_entity::users::users::{UserType, UsersEntity};
use crate::Module;

impl Module {
    pub async fn users_apply(&self, username: String, email: String, passwd: String) -> anyhow::Result<ExecResult> {
        let passwd = sha256::digest(passwd);
        let user = UsersEntity{
            uid: Default::default(),
            username: username.to_lowercase(),
            name: username.clone(),
            email,
            hide_email: false,
            passwd,
            user_type: UserType::Unused,
            local: "".to_string(),
            websites: "".to_string(),
            rands: "".to_string(),
            language: "".to_string(),
            bio: "".to_string(),
            is_active: true,
            is_admin: false,
            max_repo_count: 65545,
            avatar_url: "".to_string(),
            nums_flowers: vec![],
            nums_flowings: vec![],
            nums_stars: vec![],
            theme: "".to_string(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };
        let result = UsersEntity::insert(&self.db, &user).await?;
        Ok(result)
    }
}