use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::org::team::TeamEntity;
use jzfs_entity::uuid::Uuid;
use crate::Module;

impl Module {
    pub async fn org_team_get_by_name(&self, name: String) -> anyhow::Result<Vec<TeamEntity>>{
        let resp = TeamEntity::select_by_name(&self.db, name).await?;
        Ok(resp)
    }
    pub async fn org_team_get_by_id(&self, id: Uuid) -> anyhow::Result<TeamEntity>{
        let resp = TeamEntity::select_by_column(&self.db, "uid", id).await?.first().map(|x| x.clone());
        if resp.is_none() {
            return Err(anyhow!("[Service 10004] Team not found"));
        }
        Ok(resp.unwrap())
    }
    pub async fn org_team_get_by_owner(&self, owner: Uuid) -> anyhow::Result<Vec<TeamEntity>>{
        let resp = TeamEntity::select_by_column(&self.db, "owner_id", owner).await?;
        Ok(resp)
    }
}