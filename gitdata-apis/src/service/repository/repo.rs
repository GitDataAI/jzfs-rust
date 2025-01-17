use gitdata::model::repository::branch;
use gitdata::model::repository::commit;
use gitdata::model::repository::repository;
use gitdata::model::repository::tree;
use gitdata::model::users::stars;
use gitdata::model::users::users;
use gitdata::model::users::watch;
use gitdata::rpc::core_git::RepositoryAddFileRequest;
use gitdata::rpc::core_git::RepositoryStoragePosition;
use sea_orm::prelude::Uuid;
use sea_orm::*;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;
use crate::service::rpc::CoreGitRpc;

#[derive(Deserialize, Serialize)]
pub struct RepoCreateParam {
    pub name : String,
    pub owner_uid : Uuid,
    pub description : Option<String>,
    pub visible : bool,
    pub default_branch : Option<String>,
    pub readme : bool,
    pub node : String,
    pub message : Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RepoReNameParma {
    pub name : String,
}

#[derive(Deserialize, Serialize)]
pub struct RepoVisibleParma {
    pub visible : bool,
}
#[derive(Deserialize, Serialize)]
pub struct RepoUpdateParma {
    pub description : Option<String>,
    pub default_branch : Option<String>,
    pub website : Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RepoCreateOwners {
    avatar : Option<String>,
    name : String,
    uid : Uuid,
}
impl AppState {
    pub async fn repository_new(
        &self,
        user_model : users::Model,
        param : RepoCreateParam,
    ) -> anyhow::Result<()> {
        let active_model = repository::ActiveModel::new(
            param.name.clone(),
            param.owner_uid,
            param.description,
            param.visible,
            param.default_branch.clone(),
            param.node.clone(),
        );
        let txn = self.active_write.begin().await?;
        match active_model.clone().insert(&txn).await {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        };
        let mut client = match CoreGitRpc::get().await {
            Ok(client) => client.clone(),
            Err(e) => {
                txn.rollback().await?;
                return Err(e);
            }
        };
        let mut node = RepositoryStoragePosition::default();
        node.node = param.node;
        node.path = active_model.uid.unwrap().to_string();
        match client.client.create(node.clone()).await {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        };
        if param.readme {
            let bytes = format!("### {}", param.name);
            match client
                .client
                .add_file(RepositoryAddFileRequest {
                    repository_storage_position : Some(node),
                    path : "".to_string(),
                    content : bytes.into_bytes(),
                    email : user_model.main_email.clone(),
                    user : user_model.name.clone(),
                    message : param.message.unwrap_or("Create README.md".to_string()),
                    file_name : "README.md".to_string(),
                    branch : param.default_branch.unwrap_or("main".to_string()),
                })
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    txn.rollback().await?;
                    return Err(e.into());
                }
            }
        }
        txn.commit().await?;
        Ok(())
    }
    pub async fn repository_delete(&self, repo : repository::Model) -> anyhow::Result<()> {
        let txn = self.active_write.begin().await?;
        let repo = repository::Entity::find_by_uid(repo.uid)
            .one(&txn)
            .await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        repo.clone().into_active_model().delete(&txn).await?;
        let mut client = match CoreGitRpc::get().await {
            Ok(client) => client.clone(),
            Err(e) => {
                txn.rollback().await?;
                return Err(e);
            }
        };
        match client
            .client
            .delete(RepositoryStoragePosition {
                node : repo.clone().storage_node,
                path : repo.uid.to_string(),
            })
            .await
        {
            Ok(_) => {}
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        };
        branch::Entity::delete_many()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        commit::Entity::delete_many()
            .filter(commit::Column::RepositoryUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        // TODO tag delete
        tree::Entity::delete_many()
            .filter(tree::Column::RepositoryUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        stars::Entity::delete_many()
            .filter(stars::Column::RepositoryUid.eq(repo.uid))
            .exec(&txn)
            .await?;
        watch::Entity::delete_many()
            .filter(watch::Column::RepositoryUid.eq(repo.uid))
            .exec(&txn)
            .await?;

        repo.into_active_model().insert(&self.deprecated).await.ok();
        txn.commit().await?;
        Ok(())
    }
    pub async fn repository_rename(
        &self,
        repo : repository::Model,
        param : RepoReNameParma,
    ) -> anyhow::Result<()> {
        let txn = self.active_write.begin().await?;
        let repo = repository::Entity::find_by_uid(repo.uid)
            .one(&txn)
            .await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let mut repo = repo.clone().into_active_model();
        repo.name = Set(param.name);
        match repo.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        }
    }
    pub async fn repository_visible(
        &self,
        repo : repository::Model,
        param : RepoVisibleParma,
    ) -> anyhow::Result<()> {
        let txn = self.active_write.begin().await?;
        let repo = repository::Entity::find_by_uid(repo.uid)
            .one(&txn)
            .await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let mut repo = repo.clone().into_active_model();
        repo.visible = Set(param.visible);
        match repo.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        }
    }
    pub async fn repository_update(
        &self,
        repo : repository::Model,
        param : RepoUpdateParma,
    ) -> anyhow::Result<()> {
        let txn = self.active_write.begin().await?;
        let repo = repository::Entity::find_by_uid(repo.uid)
            .one(&txn)
            .await?
            .ok_or(anyhow::anyhow!("Repository not found"))?;
        let mut repo = repo.clone().into_active_model();
        if let Some(description) = param.description {
            repo.description = Set(Some(description));
        }
        if let Some(default_branch) = param.default_branch {
            repo.default_branch = Set(default_branch);
        }
        if let Some(message) = param.website {
            repo.website = Set(Some(message));
        }
        match repo.update(&txn).await {
            Ok(_) => {
                txn.commit().await?;
                Ok(())
            }
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        }
    }
    pub async fn repository_owner(
        &self,
        owner_uid : Uuid,
    ) -> anyhow::Result<Vec<repository::Model>> {
        let repo = repository::Entity::find()
            .filter(repository::Column::OwnerUid.eq(owner_uid))
            .all(&self.active_read)
            .await?;
        Ok(repo)
    }
    pub async fn repository_create_owner(
        &self,
        user_uid : Uuid,
    ) -> anyhow::Result<Vec<RepoCreateOwners>> {
        let mut result = Vec::new();
        let users_model = match users::Entity::find_by_uid(user_uid)
            .one(&self.active_read)
            .await
        {
            Ok(users_model) => users_model,
            Err(_) => return Err(anyhow::anyhow!("User not found")),
        };
        if users_model.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }
        let users_model = users_model.unwrap();
        result.push(RepoCreateOwners {
            avatar : users_model.avatar_url,
            name : users_model.username,
            uid : users_model.uid,
        });
        let orgs = match users::Entity::find()
            .filter(users::Column::Member.contains(user_uid))
            .all(&self.active_read)
            .await
        {
            Ok(orgs) => orgs,
            Err(_) => return Ok(result),
        };
        for idx in orgs {
            result.push(RepoCreateOwners {
                avatar : idx.avatar_url,
                name : idx.username,
                uid : idx.uid,
            });
        }
        Ok(result)
    }
}
