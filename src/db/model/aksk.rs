use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct AkSkModule{
    pub uid: Uuid,
    pub name: Option<String>,
    pub user_id: Uuid,
    pub value: String,
    pub create_at: OffsetDateTime,
    pub update_at: OffsetDateTime,
}


