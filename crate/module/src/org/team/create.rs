use jzfs_entity::config::anyhow;
use jzfs_entity::dto::org::team::CreateTeamReq;
use jzfs_entity::org::team::TeamEntity;
use jzfs_entity::rbatis::rbdc::db::ExecResult;
use jzfs_entity::time::OffsetDateTime;
use jzfs_entity::uuid::Uuid;
use crate::Module;

impl Module {
    pub async fn org_team_create(&self,dto: CreateTeamReq, create: Uuid) -> anyhow::Result<ExecResult>{
        let team = TeamEntity{
            uid: Uuid::new_v4(),
            name: dto.name,
            bio: dto.bio,
            repos: vec![],
            members: vec![],
            nums_repos: 0,
            nums_members: 0,
            unit: None,
            owner_id: create,
            active: true,
            create_at: OffsetDateTime::now_utc(),
            update_at: OffsetDateTime::now_utc(),
            create_by: create,
        };
        Ok(TeamEntity::insert(&self.db, &team).await?)
    }
}