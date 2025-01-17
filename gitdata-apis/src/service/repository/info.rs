use gitdata::model::repository::repository;
use gitdata::model::users::users;
use sea_orm::*;

use crate::service::AppState;

impl AppState {
    pub async fn repo_owner_name(
        &self,
        owner : String,
        repo : String,
    ) -> anyhow::Result<repository::Model> {
        let user_model = users::Entity::find_by_username(&owner)
            .one(&self.active_read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        let repo_model = repository::Entity::find_by_owner(user_model.uid)
            .filter(repository::Column::Name.eq(repo))
            .one(&self.active_read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("repository not found"))?;
        Ok(repo_model)
    }
    pub async fn repo_member(&self, repo : repository::Model) -> anyhow::Result<Vec<users::Model>> {
        let member = users::Entity::find_by_id(repo.owner_uid)
            .one(&self.active_read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("repository not found"))?;
        let member = member.member;
        let member = users::Entity::find()
            .filter(users::Column::Uid.is_in(member))
            .all(&self.active_read)
            .await?;
        Ok(member)
    }
}
