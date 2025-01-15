use gitdata::model::users::users;
use sea_orm::prelude::Uuid;

use crate::service::AppState;

impl AppState {
    pub async fn _users_info_by_uid(&self, users_uid : Uuid) -> anyhow::Result<users::Model> {
        Ok(users::Entity::find_by_uid(users_uid)
            .one(&self.active_read)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?)
    }
    pub async fn _users_info_by_username(&self, username : String) -> anyhow::Result<users::Model> {
        Ok(users::Entity::find_by_username(&username)
            .one(&self.active_read)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?)
    }
    pub async fn _users_info_by_email(&self, email : String) -> anyhow::Result<users::Model> {
        Ok(users::Entity::find_by_email(&email)
            .one(&self.active_read)
            .await?
            .ok_or(anyhow::anyhow!("User not found"))?)
    }
}
