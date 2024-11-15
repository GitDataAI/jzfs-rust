use crate::api::controller::v1::auth::UpdateAny;
use crate::db::auth_db::AUTHDB;
use crate::db::model::users;
use crate::db::model::users::Model;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use std::io;
use std::io::ErrorKind;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct AuthServer{
    db: DatabaseConnection
}

impl AuthServer {
    pub fn new() -> Self{
        let db = AUTHDB.get().unwrap().clone();
        Self{
            db
        }
    }
    pub async fn login(&self, username: String, password: String) -> io::Result<Model>{
        let password = sha256::digest(password);
        match users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .filter(users::Column::Password.eq(password))
            .one(&self.db).await{
            Ok(ok) => {
                match ok {
                    None => {
                        Err(io::Error::new(ErrorKind::NotFound, "User not found"))
                    }
                    Some(data) => {
                        Ok(data)
                    }
                }
            }
            Err(e) => {
                Err(io::Error::new(ErrorKind::Other, e))
            }
        }
    }
    pub async fn register(&self, username: String, password: String,email: String) -> io::Result<Model>{
        let password = sha256::digest(password);
        let active = users::ActiveModel{
            uid: Set(Uuid::new_v4()),
            name: Set(None),
            username: Set(username),
            password: Set(password),
            avatar_url: Set(None),
            email: Set(email),
            bio: Set(None),
            links: Set(Vec::new()),
            location: Set(None),
            time_zone: Set(None),
            language: Set(None),
            groups: Set(Vec::new()),
            create_at: Default::default(),
            update_at: Default::default(),
        };
        match active.insert(&self.db).await {
            Ok(model) => {
                Ok(model)
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn reset_passwd(&self, uid: Uuid,password: String) -> io::Result<()> {
        let password = sha256::digest(password);
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.db)
            .await;
        if model.is_err() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "DB links Error"))
        }
        let model = model.unwrap();
        if model.is_none(){
            return Err(io::Error::new(io::ErrorKind::NotFound, "User not found"))
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        active.password = Set(password.clone());
        active.update_at = Set(OffsetDateTime::now_utc());
        match active.update(&self.db).await {
            Ok(data) => {
                if data.password == password {
                    Ok(())
                }else {
                    Err(io::Error::new(io::ErrorKind::Other, "Password Error".to_string()))
                }
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    pub async fn update(&self,uid: Uuid,any:UpdateAny) -> io::Result<Model>{
        let model = users::Entity::find()
            .filter(users::Column::Uid.eq(uid))
            .one(&self.db)
            .await;
        if model.is_err() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "DB links Error"))
        }
        let model = model.unwrap();
        if model.is_none(){
            return Err(io::Error::new(io::ErrorKind::NotFound, "User not found"))
        }
        let model = model.unwrap();
        let mut active = model.into_active_model();
        if let Some(email) = any.email {
            active.email = Set(email);
        }
        if let Some(avatar_url) = any.avatar_url {
            active.avatar_url = Set(Some(avatar_url));
        }
        if let Some(bio) = any.bio {
            active.bio = Set(Some(bio));
        }
        if let Some(location) = any.location {
            active.location = Set(Some(location));
        }
        if let Some(groups) = any.groups{
            active.groups = Set(groups);
        }
        if let Some(languages) = any.language {
            active.language = Set(Some(languages));
        }
        if let Some(name) = any.name{
            active.name = Set(Some(name));
        }
        if let Some(username) = any.username{
            active.username = Set(username);
        }
        if let Some(time_zone) = any.timezone{
            active.time_zone = Set(Some(time_zone));
        }
        if let Some(links) = any.links{
            active.links = Set(links);
        }
        active.update_at = Set(OffsetDateTime::now_utc());
        match active.update(&self.db).await {
            Ok(data) => {
                Ok(data)
            }
            Err(e)=>{
                Err(io::Error::new(ErrorKind::Other,e))
            }
        }
    }
}