use log::error;
use model::users::users::UsersModel;
use crate::service::user::UserService;

impl UserService {
    pub async fn check_username(&self, username: String) -> anyhow::Result<bool>{
        let info = UsersModel::select_by_column(&self.db, "username", username).await
            .map_err(|e| {
                error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?;
        Ok(info.is_empty())
    }
    pub async fn check_email(&self, email: String) -> anyhow::Result<bool>{
        let info = UsersModel::select_by_column(&self.db, "email", email).await
            .map_err(|e| {
                error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?;
        Ok(info.is_empty())
    }
}