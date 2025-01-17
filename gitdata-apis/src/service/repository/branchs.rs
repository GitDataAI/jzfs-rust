use gitdata::model::repository::branch;
use gitdata::model::repository::commit;
use gitdata::model::repository::repository;
use gitdata::rpc::core_git::RepositoryCreateBranchRequest;
use gitdata::rpc::core_git::RepositoryDeleteBranchRequest;
use gitdata::rpc::core_git::RepositoryReNameBranchRequest;
use sea_orm::prelude::Uuid;
use sea_orm::*;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;
use crate::service::rpc::CoreGitRpc;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BranchsParma {
    pub repo_uid : Uuid,
    pub name : String,
    pub head : String,
    pub head_msg : String,
    pub head_time : i64,
    pub default : bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BranchCreateParma {
    pub branch_name : String,
    pub source_uid : Uuid,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BranchUpdateParma {
    pub branch_name : String,
    pub new_branch_name : Option<String>,
}

impl AppState {
    pub async fn repo_branchs(
        &self,
        repo : repository::Model,
    ) -> anyhow::Result<Vec<BranchsParma>> {
        let mut branchs = Vec::new();
        let bs = branch::Entity::find()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .all(&self.active_read)
            .await?;
        for b in bs {
            let commit = commit::Entity::find()
                .filter(commit::Column::RepositoryUid.eq(repo.uid))
                .filter(commit::Column::BranchUid.eq(b.uid))
                .order_by_desc(commit::Column::Time)
                .one(&self.active_read)
                .await?;
            branchs.push(BranchsParma {
                repo_uid : repo.uid,
                name : b.name,
                head : b.head,
                default : b.default_branch,
                head_msg : commit.clone().map(|x| x.msg).unwrap_or("N/A".to_string()),
                head_time : commit.map(|x| x.time).unwrap_or(0),
            });
        }
        Ok(branchs)
    }
    pub async fn repo_branch_create(
        &self,
        repo : repository::Model,
        param : BranchCreateParma,
    ) -> anyhow::Result<()> {
        let branch_model = branch::Entity::find()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .filter(branch::Column::Name.eq(param.branch_name.clone()))
            .one(&self.active_read)
            .await?;
        if branch_model.is_some() {
            return Err(anyhow::anyhow!("Branch already exists"));
        }
        let branch_model = branch_model.unwrap();
        let mut rpc = match CoreGitRpc::get().await {
            Ok(rpc) => rpc.clone(),
            Err(_) => return Err(anyhow::anyhow!("Rpc error")),
        };
        let request = RepositoryCreateBranchRequest {
            position : Option::from(repo.node()),
            branch : param.branch_name.clone(),
            from : branch_model.name,
        };
        match rpc.client.create_branch(request).await {
            Ok(_) => Ok(()),
            Err(x) => Err(anyhow::anyhow!(x)),
        }
    }
    pub async fn repo_branch_delete(
        &self,
        repo : repository::Model,
        parma : BranchUpdateParma,
    ) -> anyhow::Result<()> {
        let branch_model = branch::Entity::find()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .filter(branch::Column::Name.eq(parma.branch_name.clone()))
            .one(&self.active_read)
            .await?;
        if branch_model.is_none() {
            return Err(anyhow::anyhow!("Branch not found"));
        }
        if branch_model.unwrap().default_branch {
            return Err(anyhow::anyhow!("Default branch cannot be deleted"));
        }
        let mut rpc = match CoreGitRpc::get().await {
            Ok(rpc) => rpc.clone(),
            Err(_) => return Err(anyhow::anyhow!("Rpc error")),
        };
        let req = RepositoryDeleteBranchRequest {
            position : Option::from(repo.node()),
            branch : parma.branch_name.clone(),
        };
        match rpc.client.delete_branch(req).await {
            Ok(_) => Ok(()),
            Err(x) => Err(anyhow::anyhow!(x)),
        }
    }
    pub async fn repo_branch_update(
        &self,
        repo : repository::Model,
        parma : BranchUpdateParma,
    ) -> anyhow::Result<()> {
        let branch_model = branch::Entity::find()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .filter(branch::Column::Name.eq(parma.branch_name.clone()))
            .one(&self.active_read)
            .await?;
        if branch_model.is_none() {
            return Err(anyhow::anyhow!("Branch not found"));
        }
        if branch_model.unwrap().default_branch {
            return Err(anyhow::anyhow!("Default branch cannot be renamed"));
        };
        if parma.new_branch_name.is_none() {
            return Err(anyhow::anyhow!("New branch name cannot be empty"));
        }
        if parma.new_branch_name.clone().unwrap() == parma.branch_name {
            return Err(anyhow::anyhow!(
                "New branch name cannot be same as old branch name"
            ));
        }
        if branch::Entity::find()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .filter(branch::Column::Name.eq(parma.new_branch_name.clone().unwrap()))
            .one(&self.active_read)
            .await?
            .is_some()
        {
            return Err(anyhow::anyhow!("Branch already exists"));
        }
        let mut rpc = match CoreGitRpc::get().await {
            Ok(rpc) => rpc.clone(),
            Err(_) => return Err(anyhow::anyhow!("Rpc error")),
        };
        let req = RepositoryReNameBranchRequest {
            position : Option::from(repo.node()),
            branch : parma.branch_name.clone(),
            new_name : parma.new_branch_name.clone().unwrap(),
        };
        match rpc.client.re_name_branch(req).await {
            Ok(_) => Ok(()),
            Err(x) => Err(anyhow::anyhow!(x)),
        }
    }
    pub async fn repo_branch_activity(
        &self,
        repo : repository::Model,
        branch_name : String,
    ) -> anyhow::Result<Vec<commit::Model>> {
        let branch_model = branch::Entity::find()
            .filter(branch::Column::RepositoryUid.eq(repo.uid))
            .filter(branch::Column::Name.eq(branch_name))
            .one(&self.active_read)
            .await?;
        if branch_model.is_none() {
            return Err(anyhow::anyhow!("Branch not found"));
        }
        let commit = commit::Entity::find()
            .filter(commit::Column::RepositoryUid.eq(repo.uid))
            .filter(commit::Column::BranchUid.eq(branch_model.unwrap().uid))
            .order_by_desc(commit::Column::Time)
            .all(&self.active_read)
            .await?;
        Ok(commit)
    }
}
