use anyhow::anyhow;
use log::error;
use dto::repo::ListRepo;
use model::repos::repos::ReposModel;
use crate::service::repo::RepoService;

impl RepoService {
    pub async fn list_repo_by_uid(&self, owner_id: ListRepo) -> anyhow::Result<Vec<ReposModel>>{
        let models = ReposModel::select_by_column(
            &self.db,
            "owner_id",
            owner_id.uid
        ).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] List Repo Error")
            })?;
        Ok(models)
    }
    pub async fn search_repo_by_name(&self, name: String, offset: u64, limit: u64) -> anyhow::Result<Vec<ReposModel>>{
        let models = ReposModel::select_by_name(
            &self.db,
            name,
            offset,
            limit
        ).await
            .map_err(|x| {
                error!("{}", x);
                anyhow!("[Error] Search Repo Error")
            })?;
        Ok(models)
    }
}