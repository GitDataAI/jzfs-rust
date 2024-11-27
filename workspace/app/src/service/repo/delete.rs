use anyhow::anyhow;
use log::error;
use dto::repo::DeleteRepo;
use model::repos::repos::ReposModel;
use crate::service::repo::RepoService;

impl RepoService {
    pub async fn delete_repo(&self, repo_id: DeleteRepo) -> anyhow::Result<()>{
        ReposModel::delete_by_column(
            &self.db,
            "uid",
            repo_id.uid
        )
            .await
            .map_err(
                |x| {
                    error!("{}", x);
                    anyhow!("[Error] Delete Repo Error")
                }
            )?;
        Ok(())
    }
}