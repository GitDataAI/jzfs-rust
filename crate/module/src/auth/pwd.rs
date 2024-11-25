use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::auth::{AuthUserNamePassword, AuthRespDto};
use jzfs_entity::users::users::UsersEntity;
use crate::Module;

impl Module {
    pub async fn auth_pwd(&self, up: AuthUserNamePassword) -> anyhow::Result<AuthRespDto>{
        let username = up.username;
        let password = sha256::digest(up.password);
        let model = UsersEntity::select_by_column(
            &self.db,
            "username",
            username
        ).await?.first().map(|x| x.clone());
        if model.is_none() {
            return Err(anyhow!("[Service 10001] Username not found"));
        }
        let model = model.unwrap();
        if password != model.passwd{
            return Err(anyhow!("[Service 10002] Passwords not match"));
        }
        Ok(AuthRespDto::from(&model))
    }
}