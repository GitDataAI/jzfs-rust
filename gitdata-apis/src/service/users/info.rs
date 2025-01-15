use gitdata::model::users::users::Model;
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersInfoReplay {
    pub uid : Uuid,
    pub username : String,
    pub name : String,

    pub mindset : Option<String>,
    pub avatar_url : Option<String>,

    pub company : Option<String>,
    pub job_title : Option<String>,
    pub website : Option<String>,
    pub social : Vec<String>,
    pub bio : Option<String>,
    pub location : Option<String>,
    pub appellative : Option<String>,
    pub topic : Vec<String>,
    pub pinned : Vec<Uuid>,

    pub created_at : i64,
    pub updated_at : i64,
    pub last_used : Option<i64>,

    pub professional : bool,

    pub organize : bool,
    pub member : Vec<Uuid>,
    pub team : Vec<Uuid>,
}
impl From<Model> for UsersInfoReplay {
    fn from(model : Model) -> Self {
        Self {
            uid : model.uid,
            username : model.username,
            name : model.name,

            mindset : model.mindset,
            avatar_url : model.avatar_url,

            company : model.company,
            job_title : model.job_title,
            website : model.website,
            social : model.social,
            bio : model.bio,
            location : model.location,
            appellative : model.appellative,
            topic : model.topic,
            pinned : model.pinned,
            created_at : model.created_at,
            updated_at : model.updated_at,
            last_used : model.last_used,
            professional : model.professional,
            organize : model.organize,
            member : model.member,
            team : model.team,
        }
    }
}

impl AppState {
    pub async fn users_info_by_uid(&self, users_uid : Uuid) -> anyhow::Result<UsersInfoReplay> {
        let model = self._users_info_by_uid(users_uid).await?;
        Ok(UsersInfoReplay::from(model))
    }
    pub async fn users_info_by_username(
        &self,
        username : String,
    ) -> anyhow::Result<UsersInfoReplay> {
        let model = self._users_info_by_username(username).await?;
        Ok(UsersInfoReplay::from(model))
    }
    pub async fn users_info_by_email(&self, email : String) -> anyhow::Result<UsersInfoReplay> {
        let model = self._users_info_by_email(email).await?;
        Ok(UsersInfoReplay::from(model))
    }
}
