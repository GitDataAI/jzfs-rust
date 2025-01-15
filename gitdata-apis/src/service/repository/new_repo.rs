use sea_orm::prelude::Uuid;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;

#[derive(Deserialize, Serialize)]
pub struct RepoCreateParam {
    pub name : String,
    pub owner_uid : Uuid,
    pub description : Option<String>,
    pub visible : bool,
    pub default_branch : Option<String>,
    pub readme : bool,
}

impl AppState {
    pub async fn repository_new(&self) {
        todo!()
    }
}
