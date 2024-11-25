use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::auth::AuthRespDto;
use jzfs_entity::users::users::UsersEntity;
use crate::Module;

impl Module {
    pub async fn users_get_by_name(&self, name: String) -> anyhow::Result<Vec<AuthRespDto>>{
        let data = UsersEntity::select_by_name(&self.db, name)
            .await?
            .iter().map(|x| AuthRespDto::from(x))
            .collect::<Vec<_>>();
        Ok(data)
    }
    pub async fn users_get_by_id(&self, uid: jzfs_entity::uuid::Uuid) -> anyhow::Result<AuthRespDto>{
        let data = UsersEntity::select_by_column(&self.db, "uid", uid)
            .await?
            .first()
            .map(|x| AuthRespDto::from(x))
            .ok_or(anyhow!("[Service 10004] User not found"))?;
        Ok(data)
    }
}