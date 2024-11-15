use std::io;
use sea_orm::*;
use uuid::Uuid;
use crate::db::auth_db::AUTHDB;
use crate::db::model::{public_key, public_token};

pub struct KeysServer{
    db: DatabaseConnection
}

impl KeysServer {
    pub fn new() -> Self {
        let db = AUTHDB.get().unwrap().clone();
        Self { db }
    }
    pub async fn add_token(&self, user_id: Uuid,token: String,name: String) -> io::Result<public_token::Model>{
        let db = self.db.clone();
        let model = public_token::Entity::find()
            .filter(public_token::Column::Token.eq(token.clone()))
            .one(&db).await.unwrap();
        if model.is_some() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Token already exists"));
        }
        let arch = public_token::ActiveModel{
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            name: Set(name),
            token: Set(token),
            lastuse_at: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        match arch.insert(&db).await {
            Ok(model) => {
                Ok(model)
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn add_public_key(&self, user_id: Uuid,public_key: String,name: String ) -> io::Result<public_key::Model>{
        let db = self.db.clone();
        let model = public_key::Entity::find()
            .filter(public_token::Column::Token.eq(public_key.clone()))
            .one(&db).await.unwrap();
        if model.is_some() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Token already exists"));
        }
        let arch = public_key::ActiveModel{
            uid: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            name: Set(name),
            public_key: Set(public_key),
            lastuse_at: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        match arch.insert(&db).await {
            Ok(model) => {
                Ok(model)
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn del_public_key(&self,user_id: Uuid, public_key: String) -> io::Result<DeleteResult> {
        let db = self.db.clone();
        let model = public_key::Entity::find()
        .filter(public_token::Column::Token.eq(public_key.clone()))
        .one(&db).await.unwrap();
        if model.is_none(){
            return Err(io::Error::new(io::ErrorKind::NotFound, "Public key not found"));
        }
        let model = model.unwrap();
        if model.uid != user_id{
            return Err(io::Error::new(io::ErrorKind::NotFound, "Public key not belonging to this user"));
        }
        let arch = model.into_active_model();
        match arch.delete(&db).await {
            Ok(model) => {
                Ok(model)
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn del_public_token(&self, user_id: Uuid, token: String) -> io::Result<DeleteResult> {
        let db = self.db.clone();
        let model = public_token::Entity::find()
            .filter(public_token::Column::Token.eq(token.clone()))
            .one(&db).await.unwrap();
        if model.is_none(){
            return Err(io::Error::new(io::ErrorKind::NotFound, "Public key not found"));
        }
        let model = model.unwrap();
        if model.uid != user_id{
            return Err(io::Error::new(io::ErrorKind::NotFound, "Public key not belonging to this user"));
        }
        let arch = model.into_active_model();
        match arch.delete(&db).await {
            Ok(model) => {
                Ok(model)
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn list_public_keys(&self,user_id: Uuid) -> io::Result<Vec<public_key::Model>> {
        let db = self.db.clone();
        let models = public_key::Entity::find()
            .filter(public_token::Column::UserId.eq(user_id))
            .all(&db).await.unwrap();
        Ok(models)
    }
    pub async fn list_token(&self,user_id: Uuid) -> io::Result<Vec<public_token::Model>> {
        let db = self.db.clone();
        let models = public_token::Entity::find()
            .filter(public_token::Column::UserId.eq(user_id))
            .all(&db).await.unwrap();
        Ok(models)
    }
}