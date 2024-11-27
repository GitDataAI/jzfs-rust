use rbatis::rbdc::db::ExecResult;
use uuid::Uuid;
use dto::users::{UserResetPasswd, UserResetPassword};
use model::users::users::UsersModel;
use crate::service::user::UserService;

impl UserService {
    pub async fn forget_reset(&self, dto: UserResetPassword, uid: Uuid) -> anyhow::Result<ExecResult>{
        let model = UsersModel::select_by_column(
            &self.db,
            "uid",
            uid
        ).await
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
        model.passwd = dto.password;
        let info = UsersModel::update_by_column(&self.db, &model, "uid").await
            .map_err(|e| {
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?;
        Ok(info)
    }
    pub async fn reset_password(&self, dto: UserResetPasswd, uuid: Uuid) -> anyhow::Result<ExecResult>{
        let info = UsersModel::select_by_column(
            &self.db,
            "uid",
            uuid
        ).await
            .map_err(|e| {
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?
            .first()
            .map(|x| x.clone());
        if info.is_none(){
            return Err(anyhow::anyhow!("[Error] Token Error"));
        }
        let mut info = info.unwrap();
        if info.passwd != dto.old_password{
            return Err(anyhow::anyhow!("[Error] Old Password Error"));
        }
        info.passwd = dto.new_password;
        let info = UsersModel::update_by_column(&self.db, &info, "uid").await
            .map_err(|e| {
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Service Error")
            })?;
        Ok(info)
    }
}