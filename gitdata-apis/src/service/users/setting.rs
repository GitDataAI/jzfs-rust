use gitdata::model::users::users;
use sea_orm::IntoActiveModel;
use sea_orm::Set;
use sea_orm::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::service::AppState;

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersSettingBasicParam {
    pub company : Option<String>,
    pub job_title : Option<String>,
    pub website : Option<String>,
    pub social : Option<Vec<String>>,
    pub bio : Option<String>,
    pub location : Option<String>,
    pub appellate : Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersSettingTopicParam {
    pub topic : Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersSettingAvatarParam {
    pub file_name : String,
    pub data : Vec<u8>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct UsersPinedParam {
    pub repository_uids : Vec<Uuid>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UsersMindSet {
    pub mindset : String,
}

impl AppState {
    pub async fn users_setting_basic_update(
        &self,
        users_uid : Uuid,
        param : UsersSettingBasicParam,
    ) -> anyhow::Result<()> {
        let model = users::Entity::find_by_uid(users_uid)
            .one(&self.active_read)
            .await?;
        if model.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        if let Some(company) = param.company {
            active.company = Set(Some(company));
        }
        if let Some(job_title) = param.job_title {
            active.job_title = Set(Some(job_title));
        }
        if let Some(website) = param.website {
            active.website = Set(Some(website));
        }
        if let Some(social) = param.social {
            active.social = Set(social);
        }
        if let Some(bio) = param.bio {
            active.bio = Set(Some(bio));
        }
        if let Some(location) = param.location {
            active.location = Set(Some(location));
        }
        if let Some(appellate) = param.appellate {
            active.appellative = Set(Some(appellate));
        }
        active.updated_at = Set(chrono::Utc::now().timestamp());
        active.update(&self.active_write).await?;
        Ok(())
    }
    pub async fn users_setting_topic_update(
        &self,
        users_uid : Uuid,
        param : UsersSettingTopicParam,
    ) -> anyhow::Result<()> {
        let model = users::Entity::find_by_uid(users_uid)
            .one(&self.active_read)
            .await?;
        if model.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        active.topic = Set(param.topic);
        active.updated_at = Set(chrono::Utc::now().timestamp());
        active.update(&self.active_write).await?;
        Ok(())
    }
    pub async fn users_setting_avatar_update(
        &self,
        users_uid : Uuid,
        _param : UsersSettingAvatarParam,
    ) -> anyhow::Result<()> {
        let model = users::Entity::find_by_uid(users_uid)
            .one(&self.active_read)
            .await?;
        if model.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        // TODO Avatar File Upload should is a static server
        active.updated_at = Set(chrono::Utc::now().timestamp());
        active.update(&self.active_write).await?;
        Ok(())
    }
    pub async fn users_setting_pined_update(
        &self,
        users_uid : Uuid,
        param : UsersPinedParam,
    ) -> anyhow::Result<()> {
        let model = users::Entity::find_by_uid(users_uid)
            .one(&self.active_read)
            .await?;
        if model.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        active.pinned = Set(param.repository_uids);
        active.updated_at = Set(chrono::Utc::now().timestamp());
        active.update(&self.active_write).await?;
        Ok(())
    }
    pub async fn users_setting_mindset_update(
        &self,
        users_uid : Uuid,
        param : UsersMindSet,
    ) -> anyhow::Result<()> {
        let model = users::Entity::find_by_uid(users_uid)
            .one(&self.active_read)
            .await?;
        if model.is_none() {
            return Err(anyhow::anyhow!("User not found"));
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        active.mindset = Set(Option::from(param.mindset));
        active.updated_at = Set(chrono::Utc::now().timestamp());
        active.update(&self.active_write).await?;
        Ok(())
    }
}
