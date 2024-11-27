use uuid::Uuid;
use model::users::users::UsersModel;
use crate::service::user::UserService;

impl UserService {
    pub async fn point_user_by_email(&self, email: String) -> anyhow::Result<Uuid>{
        let user = UsersModel::select_by_column(&self.db, "email", email).await
            .map_err(|e| {
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?
            .first()
            .map(|x| x.clone());
        if user.is_none(){
            return Err(anyhow::anyhow!("[Error] User Not Found"));
        }
        Ok(user.unwrap().uid)
    }
}