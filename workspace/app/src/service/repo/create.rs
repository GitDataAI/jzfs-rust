use anyhow::anyhow;
use log::error;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Uuid;
use dto::repo::CreateRepo;
use model::repos::repos::ReposModel;
use crate::service::repo::RepoService;

impl RepoService {
    pub async fn create_repo(&self, dto: CreateRepo) -> anyhow::Result<ExecResult>{ 
        let model = ReposModel{
            uid: Uuid(uuid::Uuid::new_v4().to_string()),
            name: dto.name,
            bio: dto.bio,
            avatar: dto.avatar,
            owner_id: Uuid(dto.owner_id.to_string()),
            owner_group: dto.owner_group,
            private: dto.private,
            fork: false,
            fork_from: None,
            created_at: rbatis::rbdc::timestamp::Timestamp(fastdate::DateTime::now().unix_timestamp()),
            updated_at: rbatis::rbdc::timestamp::Timestamp(fastdate::DateTime::now().unix_timestamp()),
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