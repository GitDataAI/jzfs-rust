use rbatis::crud;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct CommitModel{
    pub uid: Uuid,
    pub hash: String,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    pub commit_at: String,
    pub commit_email: String,
    pub commit_msg: String,
    pub commit_time: chrono::DateTime<chrono::Utc>,
    // TODO commit Object
}


crud!(CommitModel{}, "commits");