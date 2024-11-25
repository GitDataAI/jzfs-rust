use jzfs_entity::config::anyhow;
use jzfs_entity::dto::repo::RepoFork;
use jzfs_entity::rbatis::rbdc::db::ExecResult;
use jzfs_entity::repos::repo::RepoEntity;
use jzfs_entity::time::OffsetDateTime;
use jzfs_entity::uuid::Uuid;
use crate::Module;

impl Module {
    pub async fn repo_fork(&self, fork: RepoFork,create_by: Uuid) -> anyhow::Result<ExecResult>{
        let owner_id = if fork.is_group {
            fork.group_id.unwrap()
        }else{
            create_by
        };
        let owner_name = if fork.is_group {
            self.org_team_get_by_id(owner_id).await?.name
        }else{
            self.users_get_by_id(owner_id).await?.name
        };
        let model = RepoEntity{
            uid: Uuid::new_v4(),
            owner_id,
            owner_name,
            group: fork.is_group,
            name: fork.name,
            bio: fork.bio,
            website: "".to_string(),
            default_branch: "".to_string(),
            wikipedia: "".to_string(),
            avatar_url: None,
            visible: fork.visible,
            nums_stars: 0,
            nums_forks: 0,
            nums_issues: 0,
            nums_closed_issues: 0,
            nums_open_issues: 0,
            nums_close_pull_requests: 0,
            nums_open_pull_requests: 0,
            is_empty: true,
            is_archived: false,
            is_mirrored: false,
            is_temporary: false,
            is_fork: true,
            fork_id: Some(fork.fork_id),
            git_size: 0.0,
            topic: vec![],
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            created_by: create_by,
        };
        match RepoEntity::insert(&self.db, &model).await {
            Ok(data) => {
                Ok(data)
            },
            Err(e) => {
                Err(anyhow::anyhow!("{}",e))
            }
        }
        // TODO Object Fork
    }
}