use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone)]
pub struct CommitModule{
    pub uid: String,
    pub hash: String,
    pub author: String,
    pub message: String,
    pub repo_id: String,
    pub branch_id: String,
}