use rbatis::crud;
use serde::{Deserialize, Serialize};
use crate::uuid::Uuid;

#[derive(Deserialize,Serialize)]
pub struct StarEntity{
    pub uid: Uuid,
    pub user_id: Uuid,
    pub star: usize,
    pub star_id: Vec<Uuid>
}

impl StarEntity {
    pub fn map() -> Self{
        Self{
            uid: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            star: 0,
            star_id: vec![],
        }
    }
}

crud!(StarEntity{},"stars");