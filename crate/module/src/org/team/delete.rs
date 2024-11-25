use crate::Module;
use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::org::team::TeamDeleteReq;
use jzfs_entity::org::team::TeamEntity;
use jzfs_entity::rbatis::rbdc::db::ExecResult;
use jzfs_entity::uuid::Uuid;

impl Module {
    pub async fn org_team_delete(&self, dto: TeamDeleteReq,req: Uuid) -> anyhow::Result<ExecResult> {
        let model = TeamEntity::select_by_column(
            &self.db,
            "uid",
            dto.org_id
        ).await?.first().map(|x| x.clone());
        if model.is_none() {
            return Err(anyhow!("[Service 10004] Team not found"));
        }
        let model = model.unwrap();
        if model.owner_id != dto.apply_id {
            return Err(anyhow!("[Service 10005] You are not the owner of this team"));
        }
        if req != model.owner_id {
            return Err(anyhow!("[Service 10007] You are not the owner of this team"));
        }
        if let Ok(ok) = TeamEntity::delete_by_column(&self.db, "org_id", dto.org_id).await {
            Ok(ok)
        } else {
            Err(anyhow!("[Service 10006] Delete team failed"))
        }
    }
}