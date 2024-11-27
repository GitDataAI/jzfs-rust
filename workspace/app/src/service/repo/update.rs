use anyhow::anyhow;
use log::error;
use dto::repo::UpdateRepo;
use model::repos::repos::ReposModel;
use crate::service::repo::RepoService;

impl RepoService {
    pub async fn update_repo(&self, dto: UpdateRepo) -> anyhow::Result<()>{
        let uid = dto.uid;
        let repo = ReposModel::select_by_column(
            &self.db,
            "uid",
            uid
        )
            .await
            .map_err(
                |x| {
                    error!("{}", x);
                    anyhow!("[Error] Update Repo Error")
                }
            )?
            .first()
            .map(|x| x.clone());
        if repo.is_none() {
            return Err(anyhow!("[Error] Repo Not Found"));
        }
        let mut repo = repo.unwrap();
        if let Some(name) = dto.name {
            repo.name = name;
        }
        if let Some(bio) = dto.bio {
            repo.bio = bio;
        }
        if let Some(avatar) = dto.avatar {
            repo.avatar = avatar;
        }
        if let Some(private) = dto.private {
            repo.private = private;
        }
        ReposModel::update_by_column(
            &self.db,
            &repo,
            "uid"
        )
            .await
            .map_err(
                |x| {
                    error!("{}", x);
                    anyhow!("[Error] Update Repo Error")
                }
            )?;
        Ok(())
    }
}