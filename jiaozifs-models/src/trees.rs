use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Json;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use jiaozifs_utlis::Hash;
use crate::filemode::FileMode;


#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize)]
pub struct TreeEntity{
    pub name: String,
    pub id_dir: bool,
    pub hash: Vec<u8>
}


#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct Trees {
    pub hash: Vec<u8>,
    pub repository_id: Uuid,
    pub check_sum: Option<Vec<u8>>,
    pub r#type: String,
    pub size: Option<i64>,
    pub properties: Json<FileMode>,
    pub sub_object: Vec<TreeEntity>,
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
            sub_object: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
impl Trees {
    pub fn id(&mut self,id: Vec<u8>) -> &mut Self{
        self.hash = id;
        self
    }
    pub fn repository_id(&mut self,repository_id: Uuid) -> &mut Self{
        self.repository_id = repository_id;
        self
    }
    pub fn check_sum(&mut self,check_sum: Option<Vec<u8>>) -> &mut Self{
        self.check_sum = check_sum;
        self
    }
    pub fn r#type(&mut self,r#type: String) -> &mut Self{
        self.r#type = r#type;
        self
    }
    pub fn size(&mut self,size: Option<i64>) -> &mut Self{
        self.size = size;
        self
    }
}
