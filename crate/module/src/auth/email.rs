use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::auth::{AuthEmailPassword, AuthRespDto};
use jzfs_entity::users::users::UsersEntity;
use crate::Module;

impl Module {
    pub async fn auth_email(&self, emails: AuthEmailPassword) -> anyhow::Result<AuthRespDto>{
        let email = emails.email;
        let password = sha256::digest(emails.password);
        let model = UsersEntity::select_by_column(
            &self.db,
            "email",
            email
        ).await?.first().map(|x| x.clone());
        if model.is_none() {
            return Err(anyhow!("[Service 10003] email not found"));
        }
        let model = model.unwrap();
        if password != model.passwd{
            return Err(anyhow!("[Service 10002] Passwords not match"));
        }
        Ok(AuthRespDto::from(&model))
    }
}