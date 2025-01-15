use gitdata::model::users::users;
use sea_orm::ActiveModelTrait;
use sea_orm::Set;
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersApplyParam {
    pub username : String,
    pub email : String,
    pub passwd : String,
}

impl AppState {
    pub async fn users_check_already_exists_username(
        &self,
        username : String,
    ) -> anyhow::Result<bool> {
        Ok(users::Entity::find_by_username(&username)
            .one(&self.active_read)
            .await?
            .is_some())
    }
    pub async fn users_check_already_exists_email(&self, email : String) -> anyhow::Result<bool> {
        Ok(users::Entity::find_by_email(&email)
            .one(&self.active_read)
            .await?
            .is_some())
    }
    pub async fn users_check_already_exists_uid(&self, uid : Uuid) -> anyhow::Result<bool> {
        Ok(users::Entity::find_by_uid(uid)
            .one(&self.active_read)
            .await?
            .is_some())
    }
    pub async fn users_apply(&self, param : UsersApplyParam) -> anyhow::Result<()> {
        match self
            .users_check_already_exists_username(param.username.clone())
            .await
        {
            Ok(true) => {
                return Err(anyhow::anyhow!("Username already exists"));
            }
            Err(err) => {
                return Err(err);
            }
            _ => {}
        };
        match self
            .users_check_already_exists_email(param.email.clone())
            .await
        {
            Ok(true) => {
                return Err(anyhow::anyhow!("Email already exists"));
            }
            Err(err) => {
                return Err(err);
            }
            _ => {}
        };
        let mut model = users::ActiveModel::new_users(param.username, param.email, param.passwd);
        model.state = Set("Active".to_string());
        loop {
            match self
                .users_check_already_exists_uid(model.uid.clone().unwrap())
                .await
            {
                Ok(true) => {
                    model.uid = Set(Uuid::new_v4());
                }
                Err(err) => {
                    return Err(err);
                }
                _ => {
                    break;
                }
            };
        }
        model.insert(&self.active_write).await?;
        Ok(())
    }
}
