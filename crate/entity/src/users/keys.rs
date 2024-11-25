use rbatis::crud;
use serde::{Deserialize, Serialize};
use crate::uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct UserKeysEntity{
    pub uid: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub public_key: String,
}
impl UserKeysEntity {
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "".to_string(),
            public_key: "".to_string(),
        }
    }
}
crud!(UserKeysEntity{}, "user_keys");