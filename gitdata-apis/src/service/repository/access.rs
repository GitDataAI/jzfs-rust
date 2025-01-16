use sea_orm::*;
use gitdata::model::repository::repository;
use gitdata::model::users::{ssh_keys, token_key, users};
use crate::service::AppState;

impl AppState {
    pub async fn repo_access_token(&self, repo: repository::Model, tk: String) -> anyhow::Result<token_key::Model>{
        let owner_model = users::Entity::find_by_uid(repo.owner_uid)
            .one(&self.active_read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        let tokens = if owner_model.organize {
            let user_uids = users::Entity::find()
                .filter(users::Column::Uid.is_in(owner_model.member))
                .all(&self.active_read)
                .await
                .map(|users| users.into_iter().map(|user| user.uid).collect::<Vec<_>>())?;
            token_key::Entity::find()
                .filter(token_key::Column::UserUid.is_in(user_uids))
                .all(&self.active_read)
                .await
                .map_err(|_| anyhow::anyhow!("token not found"))?
        } else {
            match token_key::Entity::find()
                .filter(token_key::Column::UserUid.eq(owner_model.uid))
                .all(&self.active_read)
                .await{
                Ok(token) => token,
                Err(_) => return Err(anyhow::anyhow!("token not found"))
            }
        };
        for token in tokens {
            if token.token == tk {
                return Ok(token);
            }
        }
        Err(anyhow::anyhow!("token not found"))
    } 
    pub async fn repo_access_ssh(&self, repo: repository::Model, pbk: String) -> anyhow::Result<ssh_keys::Model>{
        let owner_model = users::Entity::find_by_uid(repo.owner_uid)
            .one(&self.active_read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;
        let keys = if owner_model.organize {
            let user_uids = users::Entity::find()
                .filter(users::Column::Uid.is_in(owner_model.member))
                .all(&self.active_read)
                .await
                .map(|users| users.into_iter().map(|user| user.uid).collect::<Vec<_>>())?;
            ssh_keys::Entity::find()
                .filter(ssh_keys::Column::UserUid.is_in(user_uids))
                .all(&self.active_read)
                .await
                .map_err(|_| anyhow::anyhow!("ssh key not found"))?
        }else { 
            match ssh_keys::Entity::find()
                .filter(ssh_keys::Column::UserUid.eq(owner_model.uid))
                .all(&self.active_read)
                .await{
                Ok(key) => key,
                Err(_) => return Err(anyhow::anyhow!("ssh key not found"))
            }
        };
        for key in keys {
            if key.ssh_key == pbk {
                return Ok(key);
            }
        }
        Err(anyhow::anyhow!("ssh key not found"))
    }
}