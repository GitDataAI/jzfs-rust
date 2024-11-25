use rbatis::crud;
use crate::time::OffsetDateTime;
use crate::uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepoEntity{
    pub uid: Uuid,
    pub owner_id: Uuid,
    pub owner_name: String,
    pub group: bool,
    pub name: String,
    pub bio: String,
    pub website: String,
    pub default_branch: String,
    pub wikipedia: String,
    pub avatar_url: Option<String>,
    pub visible: bool,
    pub nums_stars: i32,
    pub nums_forks: i32,
    pub nums_issues: i32,
    pub nums_closed_issues: i32,
    pub nums_open_issues: i32,
    pub nums_close_pull_requests: i32,
    pub nums_open_pull_requests: i32,

    pub is_empty: bool,
    pub is_archived: bool,
    pub is_mirrored: bool,
    pub is_temporary: bool,

    pub is_fork: bool,
    pub fork_id: Option<Uuid>,

    pub git_size: f32,
    pub topic: Vec<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub created_by: Uuid,
}
impl RepoEntity {
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            owner_name: "".to_string(),
            group: false,
            name: "".to_string(),
            bio: "".to_string(),
            website: "".to_string(),
            default_branch: "".to_string(),
            wikipedia: "".to_string(),
            avatar_url: None,
            visible: false,
            nums_stars: 0,
            nums_forks: 0,
            nums_issues: 0,
            nums_closed_issues: 0,
            nums_open_issues: 0,
            nums_close_pull_requests: 0,
            nums_open_pull_requests: 0,
            is_empty: false,
            is_archived: false,
            is_mirrored: false,
            is_temporary: false,
            is_fork: false,
            fork_id: None,
            git_size: 0.0,
            topic: vec![],
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
            created_by: Uuid::new_v4(),
        }
    }
}

crud!(RepoEntity{},"repos");
