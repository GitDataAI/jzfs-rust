use gitdata::model::repository::repository;
use gitdata::model::users::ssh_keys;
use sea_orm::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;

#[derive(Deserialize, Serialize, Clone)]
pub struct SshKeysCreateParam {
    pub name : String,
    pub description : Option<String>,
    pub ssh_key : String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SShKeysListReply {
    pub uid : Uuid,
    pub user_uid : Uuid,
    pub name : String,
    pub description : Option<String>,
    pub created_at : i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SshKeyDeleteParam {
    pub ssh_key_uid : Uuid,
}

impl AppState {
    pub async fn users_ssh_check_name(
        &self,
        users_uid : Uuid,
        name : String,
    ) -> anyhow::Result<bool> {
        Ok(ssh_keys::Entity::find_by_user_uid(users_uid)
            .filter(ssh_keys::Column::Name.eq(name))
            .one(&self.active_read)
            .await?
            .is_some())
    }
    pub async fn users_ssh_check_key(&self, ssh_key : String) -> anyhow::Result<bool> {
        Ok(!ssh_keys::Entity::find_by_ssh_key(ssh_key)
            .all(&self.active_read)
            .await?
            .is_empty())
    }
    pub async fn users_ssh_key_create(
        &self,
        users_uid : Uuid,
        param : SshKeysCreateParam,
    ) -> anyhow::Result<()> {
        match self
            .users_ssh_check_name(users_uid, param.name.clone())
            .await
        {
            Ok(true) => {
                return Err(anyhow::anyhow!("Name already exists"));
            }
            Err(err) => {
                return Err(err);
            }
            _ => {}
        };
        match self.users_ssh_check_key(param.ssh_key.clone()).await {
            Ok(true) => {
                return Err(anyhow::anyhow!("Key already exists"));
            }
            Err(err) => {
                return Err(err);
            }
            _ => {}
        };
        let model = ssh_keys::ActiveModel::new_ssh_key(
            users_uid,
            param.name,
            param.description,
            param.ssh_key,
        );
        model.insert(&self.active_write).await?;
        Ok(())
    }
    pub async fn users_ssh_key_list(
        &self,
        users_uid : Uuid,
    ) -> anyhow::Result<Vec<SShKeysListReply>> {
        Ok(ssh_keys::Entity::find_by_user_uid(users_uid)
            .all(&self.active_read)
            .await?
            .iter()
            .map(|model| SShKeysListReply {
                uid : model.uid,
                user_uid : model.user_uid,
                name : model.name.clone(),
                description : model.description.clone(),
                created_at : model.created_at,
            })
            .collect())
    }
    pub async fn users_ssh_key_delete(
        &self,
        users_uid : Uuid,
        param : SshKeyDeleteParam,
    ) -> anyhow::Result<()> {
        match ssh_keys::Entity::find_by_id(param.ssh_key_uid)
            .one(&self.active_read)
            .await?
        {
            Some(model) => {
                if model.user_uid != users_uid {
                    return Err(anyhow::anyhow!("Key not found"));
                }
                model.delete(&self.active_write).await?;
                Ok(())
            }
            None => Err(anyhow::anyhow!("Key not found")),
        }
    }

    pub async fn users_ssh_key_rpc_list(
        &self,
        ssh_keys : String,
    ) -> anyhow::Result<Vec<repository::Model>> {
        let ssh_keys = ssh_keys::Entity::find_by_ssh_key(ssh_keys)
            .one(&self.active_read)
            .await?;
        if let Some(ssh_key) = ssh_keys {
            let repository = repository::Entity::find_by_owner(ssh_key.user_uid)
                .all(&self.active_read)
                .await?;
            Ok(repository)
        } else {
            Err(anyhow::anyhow!("Key not found"))
        }
    }
}
