use anyhow::anyhow;
use log::error;
use rbatis::rbdc::db::ExecResult;
use uuid::Uuid;
use dto::repo::CreateRepo;
use model::repos::repos::ReposModel;
use crate::service::repo::RepoService;

impl RepoService {
    pub async fn create_repo(&self, dto: CreateRepo) -> anyhow::Result<ExecResult>{ 
        let model = ReposModel{
            uid: Uuid::new_v4(),
            name: dto.name,
            bio: dto.bio,
            avatar: dto.avatar,
            owner_id: dto.owner_id,
            owner_group: dto.owner_group,
            private: dto.private,
            fork: false,
            fork_from: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        Ok(ReposModel::insert(
            &self.db,
            &model
        ).await
               .map_err(|x|{
                   error!("{}", x);
                   anyhow!("[Error] Create Repo Error")
               })?)
    }
}