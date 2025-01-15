use gitdata::model::users::users;
use serde::Deserialize;
use sha256::Sha256Digest;

use crate::service::AppState;
use crate::service::users::info::UsersInfoReplay;

#[derive(Deserialize, Clone)]
pub struct UsersAuthPasswdParam {
    pub username : String,
    pub password : String,
}

impl AppState {
    pub async fn auth_by_passwd(
        &self,
        param : UsersAuthPasswdParam,
    ) -> anyhow::Result<UsersInfoReplay> {
        let name = param.username.clone();
        let passwd = param.password.clone();

        let model = if let Some(model) = users::Entity::find_by_username(&name)
            .one(&self.active_read)
            .await?
        {
            model
        } else if let Some(model) = users::Entity::find_by_email(&name)
            .one(&self.active_read)
            .await?
        {
            model
        } else {
            return Err(anyhow::anyhow!("User not found"));
        };
        if model.hash_pass == passwd.digest() {
            if model.organize {
                return Err(anyhow::anyhow!("User Forbid Login"));
            }
            if model.state != "Active" {
                return Err(anyhow::anyhow!(
                    "User Forbid Login Please Active the Account"
                ));
            }
            Ok(UsersInfoReplay::from(model))
        } else {
            Err(anyhow::anyhow!("Password error"))
        }
    }
}
