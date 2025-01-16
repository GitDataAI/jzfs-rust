use crate::service::core_git_rpc::CoreGitRpc;
use crate::service::AppState;
use gitdata::model::repository::repository;
use gitdata::model::users::users;
use gitdata::rpc::core_git::{RepositoryAddFileRequest, RepositoryCreate, RepositoryStoragePosition, RepositoryStoragePositionType};
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct RepoCreateParam {
    pub name : String,
    pub owner_uid : Uuid,
    pub description : Option<String>,
    pub visible : bool,
    pub default_branch : Option<String>,
    pub readme : bool,
    pub node: String,
    pub storage_position : i32,
    pub message: Option<String>,
}

impl AppState {
    pub async fn repository_new(&self, user_model: users::Model, param : RepoCreateParam) -> anyhow::Result<()> {
        let active_model = repository::ActiveModel::new(
            param.name.clone(),
            param.owner_uid,
            param.description,
            param.visible,
            param.default_branch.clone(),
        );
        let txn = self.active_write.begin().await?;
        match active_model.clone().insert(&txn).await{
            Ok(_) => {},
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into())
            }
        };
        let mut client = match CoreGitRpc::get().await{
            Ok(client) => client.clone(),
            Err(e) => {
                txn.rollback().await?;
                return Err(e);
            }
        };
        let mut node = RepositoryStoragePosition::default();
        node.node = param.node;
        node.path = active_model.uid.unwrap().to_string();
        node.r#type = match RepositoryStoragePositionType::try_from(param.storage_position){
            Ok(t) => i32::from(t),
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into());
            }
        };
        match client.client.create(RepositoryCreate {
            storage_position: Some(node.clone()),
        }).await {
            Ok(_) => {
            },
            Err(e) => {
                txn.rollback().await?;
                return Err(e.into())
            }
        };
        if param.readme {
            let bytes = format!("### {}", param.name);
            match client.client.add_file(RepositoryAddFileRequest {
                repository_storage_position: Some(node),
                path: "/".to_string(),
                content: bytes.into_bytes(),
                email: user_model.main_email.clone(),
                user: user_model.name.clone(),
                message: param.message.unwrap_or("Create README.md".to_string()),
                file_name: "README.md".to_string(),
                branch: param.default_branch.unwrap_or("main".to_string()),
           }).await {
                Ok(_) => {},
                Err(e) => {
                    txn.rollback().await?;
                    return Err(e.into())
                }
            }
        }
        txn.commit().await?;
        Ok(())
    }
}
