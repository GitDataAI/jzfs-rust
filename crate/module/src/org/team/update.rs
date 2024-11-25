use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::org::team::TeamUpdateReq;
use jzfs_entity::org::team::TeamEntity;
use jzfs_entity::uuid::Uuid;
use crate::Module;

impl Module {
    pub async fn org_team_update(&self, dto:  TeamUpdateReq, req: Uuid) -> anyhow::Result<()>{
        let model = TeamEntity::select_by_column(
            &self.db,
            "uid",
            dto.org_id
        ).await?.first().map(|x| x.clone());
        if model.is_none() {
            return Err(anyhow!("[Service 10004] Team not found"));
        }
        let mut model = model.unwrap();
        if req != model.uid{
            return Err(anyhow!("[Service 10005] You are not the owner of this team"));
        }
        if let Some(name) = dto.name {
            model.name = name;
            TeamEntity::update_by_column(&self.db, &model, "name").await?;
        }
        if let Some(bio) = dto.bio {
            model.bio = bio;
            TeamEntity::update_by_column(&self.db, &model, "bio").await?;
        }
        Ok(())
    }
}