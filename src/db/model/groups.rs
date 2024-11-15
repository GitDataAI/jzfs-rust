use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone)]
pub struct GroupModule{
    pub uid: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: String,
    pub location: String,
    pub description: String,
    pub users: Vec<Uuid>,
}