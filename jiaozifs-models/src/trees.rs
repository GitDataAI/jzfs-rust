use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::filemode::FileMode;

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct Trees {
    pub hash: Vec<u8>,
    pub repository_id: Uuid,
    pub check_sum: Option<Vec<u8>>,
    pub r#type: String,
    pub size: Option<i64>,
    pub properties: Json<FileMode>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}


impl Default for Trees {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Trees {
            hash: vec![],
            repository_id: Uuid::nil(),
            check_sum: None,
            r#type: "tree".to_string(),
            size: None,
            properties: Json(FileMode::Empty),
            created_at: now,
            updated_at: now,
        }
    }
}