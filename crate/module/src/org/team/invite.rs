use jzfs_entity::config::anyhow;
use jzfs_entity::config::anyhow::anyhow;
use jzfs_entity::dto::org::team::TeamInviteUsers;
use jzfs_entity::org::team::TeamEntity;
use jzfs_entity::org::team_invite::TeamUserInvite;
use jzfs_entity::rbatis::rbdc::db::ExecResult;
use jzfs_entity::time::OffsetDateTime;
use jzfs_entity::users::users::UsersEntity;
use jzfs_entity::uuid::Uuid;
use crate::Module;

impl Module {
    pub async fn org_team_invite(&self, dto: TeamInviteUsers, req: Uuid) -> anyhow::Result<ExecResult>{
        let email = dto.user_email;
        let user_model = UsersEntity::select_by_column(
            &self.db,
            "email",
            email
        ).await?.first().map(|x| x.clone());
        if user_model.is_none() {
            return Err(anyhow!("[Service 10004] User not found"));
        }
        let user_model = user_model.unwrap();
        let item_model = TeamEntity::select_by_column(
            &self.db,
            "uid",
            dto.team_id
        ).await?.first().map(|x| x.clone());
        if item_model.is_none() {
            return Err(anyhow!("[Service 10005] Item Not Found"));
        }
        let item_model = item_model.unwrap();
        if item_model.owner_id != req {
            return Err(anyhow!("[Service 10006] You are not the owner of this team"));
        }
        let invite = TeamUserInvite::select_by_column(
            &self.db,
            "user_id",
            user_model.uid
        ).await?
            .iter()
            .map(|x| x.clone())
            .filter(|x| x.team_id == item_model.uid)
            .collect::<Vec<_>>();
        if invite.len() > 0 {
            return Err(anyhow!("[Service 10007] User has been invited"));
        }
        let item = TeamUserInvite{
            uid: Uuid::new_v4(),
            team_id: dto.team_id,
            user_id: user_model.uid,
            create_at: OffsetDateTime::now_utc(),
            create_by: req,
            allow: false,
            active: true,
        };
        // TODO send email and allow url
        Ok(TeamUserInvite::insert(&self.db, &item).await?)
    }
}