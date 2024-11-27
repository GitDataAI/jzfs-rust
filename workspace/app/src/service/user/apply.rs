use std::collections::HashMap;
use log::error;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::JsonV;
use dto::users::UserApply;
use model::users::users::UsersModel;
use crate::service::user::UserService;

impl UserService {
    pub async fn apply(&self, dto: UserApply) -> anyhow::Result<ExecResult>{
        let model = UsersModel{
            uid: Default::default(),
            username: dto.username.clone(),
            email: dto.email,
            phone: None,
            passwd: dto.password,
            bio: None,
            name: dto.username,
            pro: false,
            active: true,
            repo: JsonV(HashMap::new()),
            created_at: Default::default(),
            avatar: None,
            updated_at: Default::default(),
        };
        let result = UsersModel::insert(&self.db, &model).await
            .map_err(|e|{
                error!("[Error] {}", e);
                anyhow::anyhow!("[Error] Insert User Error")
            })?;
        Ok(result)
    }
}