use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Encode,Decode,Serialize,Deserialize,Debug,Clone)]
pub struct BrandModule{
    pub uid: Uuid,
    pub repo_id: Uuid,

    pub name: String,
    pub bio: String,
    pub create_id: String,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
}