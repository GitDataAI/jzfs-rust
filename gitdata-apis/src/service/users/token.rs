use gitdata::model::repository::repository;
use gitdata::model::users::token_key;
use sea_orm::prelude::Uuid;
use sea_orm::*;
use serde::Deserialize;
use serde::Serialize;
use sha256::Sha256Digest;

use crate::service::AppState;

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenCreateParam {
    pub access : i32,
    pub name : String,
    pub description : Option<String>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct TokenCreateReply {
    pub token : String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenListParam {
    pub access : Option<i32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenListReply {
    pub uid : Uuid,
    pub name : String,
    pub description : Option<String>,
    pub user_uid : Uuid,
    pub access : i32,
    pub created_at : i64,
    pub updated_at : i64,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TokenDeleteParam {
    pub token_uid : Uuid,
}

impl AppState {
    pub async fn users_generate_token(
        &self,
        users_uid : Uuid,
        param : TokenCreateParam,
    ) -> anyhow::Result<TokenCreateReply> {
        match self._users_info_by_uid(users_uid).await {
            Ok(users) => users,
            Err(err) => {
                return Err(err);
            }
        };
        let token = format!(
            "{}{}{}",
            users_uid.to_string().digest(),
            Uuid::new_v4().to_string().digest(),
            chrono::Utc::now().to_rfc2822().digest()
        )
        .digest();
        let model = token_key::ActiveModel::new_token(
            param.name,
            param.description,
            users_uid,
            token.clone(),
            param.access,
        );
        model.insert(&self.active_write).await?;
        Ok(TokenCreateReply { token })
    }
    pub async fn users_token_list(
        &self,
        users_uid : Uuid,
        param : TokenListParam,
    ) -> anyhow::Result<Vec<TokenListReply>> {
        match self._users_info_by_uid(users_uid).await {
            Ok(users) => users,
            Err(err) => {
                return Err(err);
            }
        };
        let mut query = token_key::Entity::find().filter(token_key::Column::UserUid.eq(users_uid));
        if let Some(access) = param.access {
            query = query.filter(token_key::Column::Access.eq(access));
        };
        Ok(query
            .all(&self.active_read)
            .await?
            .iter()
            .map(|model| TokenListReply {
                uid : model.uid,
                name : model.name.clone(),
                description : model.description.clone(),
                user_uid : model.user_uid,
                access : model.access,
                created_at : model.created_at,
                updated_at : model.updated_at,
            })
            .collect::<Vec<_>>())
    }
    pub async fn users_token_delete(
        &self,
        users_uid : Uuid,
        param : TokenDeleteParam,
    ) -> anyhow::Result<()> {
        match self._users_info_by_uid(users_uid).await {
            Ok(users) => users,
            Err(err) => {
                return Err(err);
            }
        };
        match token_key::Entity::find_by_id(param.token_uid)
            .one(&self.active_read)
            .await?
        {
            Some(model) => {
                if model.user_uid != users_uid {
                    return Err(anyhow::anyhow!("Token not found"));
                }
                model.delete(&self.active_write).await?;
                Ok(())
            }
            None => Err(anyhow::anyhow!("Token not found")),
        }
    }

    pub async fn users_token_rpc_list(
        &self,
        token_requests : String,
    ) -> anyhow::Result<Vec<repository::Model>> {
        let token_requests = token_requests
            .split(":")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        if token_requests.len() < 2 {
            return Err(anyhow::anyhow!("Token not found"));
        }
        let username = token_requests[0].clone();
        let token = token_requests[1].clone();
        let users_model = match self._users_info_by_username(username).await {
            Ok(users) => users,
            Err(err) => {
                return Err(err);
            }
        };
        match token_key::Entity::find()
            .filter(token_key::Column::UserUid.eq(users_model.uid))
            .filter(token_key::Column::Token.eq(token))
            .one(&self.active_read)
            .await?
        {
            Some(_) => {
                let repository = repository::Entity::find_by_owner(users_model.uid)
                    .all(&self.active_read)
                    .await?;
                Ok(repository)
            }
            None => Err(anyhow::anyhow!("Token not found")),
        }
    }
}
