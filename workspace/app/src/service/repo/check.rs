use anyhow::anyhow;
use log::error;
use uuid::Uuid;
use model::repos::repos::ReposModel;
use crate::service::repo::RepoService;

impl RepoService {
    pub async fn check_name(&self, name: String, owner_id: Uuid) -> anyhow::Result<()>{
        let models = ReposModel::select_by_column(
            &self.db,
            "owner_id",
            owner_id
        ).await
            .map_err(|e| {
                error!("{}", e);
                anyhow!("[Error] Select Repo Name Failed")
            })?;
        for model in models {
            if model.name == name {
                return Err(anyhow!("[Error] Repo Name Exists"));
            }
        }
        Ok(())
    }
    pub async fn check_repo_owner(&self, owner_id: Uuid, repo_id: Uuid) -> anyhow::Result<()>{
        let model = ReposModel::select_by_column(
            &self.db,
            "uid",
            repo_id
        ).await
            .map_err(|e| {
                error!("{}", e);
                anyhow!("[Error] Select Repo Failed")
            })?;
        if model.len() == 0 {
            return Err(anyhow!("[Error] Repo Not Exists"));
        }
        if model[0].owner_id != owner_id {
            return Err(anyhow!("[Error] Not Repo Owner"));
        }
        Ok(())
    }
    
}