use rbatis::rbdc::db::ExecResult;
use uuid::Uuid;
use dto::users::UserUpdate;
use model::users::users::UsersModel;
use crate::service::user::UserService;

impl UserService {
    pub async fn update(&self, dto: UserUpdate, user_id: Uuid) -> anyhow::Result<Vec<ExecResult>>{
        let exec = Vec::new();
        let model = UsersModel::select_by_column(&self.db, "uid", user_id).await
            .map_err(|e| {
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?
            .first()
            .map(|x| x.clone());
        if model.is_none(){
            return Err(anyhow::anyhow!("[Error] User Not Found"));
        }
        let mut model = model.unwrap();
        if let Some(name) = dto.name {
            model.name = name;
        }
        if let Some(username) = dto.username {
            model.username = username;
        }
        if let Some(email) = dto.email {
            model.email = email;
        }
        if let Some(phone) = dto.phone {
            model.phone = Some(phone);
        }
        if let Some(bio) = dto.bio {
            model.bio = Some(bio);
        }
        if let Some(avatar) = dto.avatar {
            model.avatar = Some(avatar);
        }
        UsersModel::update_by_column(&self.db, &model, "uid").await
            .map_err(|e| {
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?;
        Ok(exec)
    }
}