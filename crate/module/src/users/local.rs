use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::auth::AuthRespDto;
use jzfs_entity::users::users::UsersEntity;
use jzfs_entity::uuid::Uuid;
use crate::Module;

impl Module {
    pub async fn users_local(&self,uid: Uuid) -> anyhow::Result<AuthRespDto>{
        let model = UsersEntity::select_by_column(&self.db, "uid", uid).await?.first().map(|x| x.clone());
        if model.is_none() {
            return Err(anyhow!("[Service 10004] User not found"));
        }
        Ok(AuthRespDto::from(&model.unwrap()))
    }
}