use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use base64::engine::general_purpose::STANDARD;
use serde::Deserialize;
use base64::prelude::*;
use serde_json::json;
use time::format_description;
use uuid::Uuid;
use crate::api::controller::session::{IsLogin, UsersModel};
use crate::api::server::auth::AuthServer;
use crate::db::model::users;

// Must Base64
#[derive(Deserialize)]
pub struct LoginBean{
    pub username: String,
    pub password: String,
}
// Must Base64
#[derive(Deserialize)]
pub struct RegisterBean{
    pub username: String,
    pub password: String,
    pub email: String,
}
// Must Base64
#[derive(Deserialize)]
pub struct UpdatePassword{
    pub password: String,
}
#[derive(Deserialize)]
pub struct UpdateAny{
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub links: Option<Vec<String>>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub groups: Option<Vec<Uuid>>,
}
#[derive(Deserialize)]
pub struct AuthBase64{
    inner: String
}
#[inline]
pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg
        .route("login", web::post().to(login))
        .route("register", web::post().to(register))
        .route("logout", web::post().to(logout))
        .route("local", web::post().to(local))
        .route("updatawp",web::post().to(update_passwd))
        .route("updata",web::post().to(update_any));
}

pub async fn login(bean: web::Json<AuthBase64>,session: Session) -> impl Responder{
    let inner = STANDARD.decode(bean.inner.as_bytes());
    if inner.is_err() {
        return HttpResponse::Accepted()
            .json(json!({
                "code": 202,
                "msg": "Invalid base64"
            }))
    }
    let bean = serde_json::from_slice::<LoginBean>(&inner.unwrap());
    if bean.is_err() {
        return HttpResponse::InternalServerError()
        .json(json!({
            "code": 500,
            "msg": "Internal server error"
        }))
    }
    let bean = bean.unwrap();
    let server = AuthServer::new();
    match server.login(bean.username, bean.password).await {
        Ok(model) => {
            let format = format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
            ).unwrap();
            let create_at = model.create_at.format(&format).unwrap();
            let update_at = model.update_at.format(&format).unwrap();
            session.insert(UsersModel,model.clone()).unwrap();
            session.insert(IsLogin,true).unwrap();
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Login successful",
                "data":{
                    "uid": model.uid,
                    "name": model.name,
                    "username": model.username,
                    "email": model.email,
                    "avatar": model.avatar_url,
                    "bio": model.bio,
                    "links": model.links,
                    "location": model.location,
                    "time_zone": model.time_zone,
                    "language": model.language,
                    "groups": model.groups,
                    "create_at": create_at,
                    "update_at": update_at,
                }
            }))
        }
        Err(e) => {
            let msg = format!("Error during login: {:?}", e);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg,
            }))
        }
    }
}
pub async fn register(bean: web::Json<AuthBase64>) -> impl Responder{
    let inner = STANDARD.decode(bean.inner.as_bytes());
    if inner.is_err() {
        return HttpResponse::Accepted()
            .json(json!({
                "code": 202,
                "msg": "Invalid base64"
            }))
    }
    let bean = serde_json::from_slice::<RegisterBean>(&inner.unwrap());
    if bean.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({
            "code": 500,
            "msg": "Internal server error"
        }))
    }
    let bean = bean.unwrap();
    let server = AuthServer::new();
    match server.register(bean.username,bean.password,bean.email).await {
        Ok(_model) => {
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Register successful",
            }))
        }
        Err(e) => {
            let msg = format!("Error during register: {:?}", e);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg,
            }))
        }
    }
}
pub async fn logout(session: Session) -> impl Responder{
    session.clear();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "code": 200,
            "msg": "Logout successful"
    }))
}
pub async fn local(session: Session) -> impl Responder{
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        return HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "code": 500,
            "msg": msg
        }))
    }
    let model = model.unwrap();
    if model.is_none() {
        return HttpResponse::Unauthorized()
        .content_type("application/json")
        .json(json!({
            "code": 401,
            "msg": "User is not logged in"
        }))
    }
    let model = model.unwrap();
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
    ).unwrap();
    let create_at = model.create_at.format(&format).unwrap();
    let update_at = model.update_at.format(&format).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
                "code": 200,
                "msg": "Login successful",
                "data":{
                    "uid": model.uid,
                    "name": model.name,
                    "username": model.username,
                    "email": model.email,
                    "avatar": model.avatar_url,
                    "bio": model.bio,
                    "links": model.links,
                    "location": model.location,
                    "time_zone": model.time_zone,
                    "language": model.language,
                    "groups": model.groups,
                    "create_at": create_at,
                    "update_at": update_at,
                }
            }
        )
    )


}
pub async fn update_passwd(bean: web::Json<AuthBase64>,session: Session) -> impl Responder{
    let inner = STANDARD.decode(bean.inner.as_bytes());
    if inner.is_err() {
        return HttpResponse::Accepted()
            .json(json!({
                "code": 202,
                "msg": "Invalid base64"
            }))
    }
    let bean = serde_json::from_slice::<UpdatePassword>(&inner.unwrap());
    if bean.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({
            "code": 500,
            "msg": "Internal server error"
        }))
    }
    let bean = bean.unwrap();
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
            "code": 500,
            "msg": msg
        }))
    }
    let model = model.unwrap();
    if model.is_none() {
        return HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(json!({
            "code": 401,
            "msg": "User is not logged in"
        }))
    }
    let model = model.unwrap();
    match AuthServer::new().reset_passwd(model.uid,bean.password).await{
        Ok(_) => {
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Reset PassWd Successful",
            }))
        }
        Err(e) => {
            let msg = format!("Error during reset passwd: {:?}", e);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg,
            }))
        }
    }

}
pub async fn update_any(bean: web::Json<UpdateAny>,session: Session) -> impl Responder{
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
            "code": 500,
            "msg": msg
        }))
    }
    let model = model.unwrap();
    if model.is_none() {
        return HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(json!({
            "code": 401,
            "msg": "User is not logged in"
        }))
    }
    let model = model.unwrap();
    match AuthServer::new().update(model.uid,bean.0).await {
        Ok(model) => {
            let format = format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
            ).unwrap();
            let create_at = model.create_at.format(&format).unwrap();
            let update_at = model.update_at.format(&format).unwrap();
            session.insert(UsersModel,model.clone()).unwrap();
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Login successful",
                 "data":{
                    "uid": model.uid,
                    "name": model.name,
                    "username": model.username,
                    "email": model.email,
                    "avatar": model.avatar_url,
                    "bio": model.bio,
                    "links": model.links,
                    "location": model.location,
                    "time_zone": model.time_zone,
                    "language": model.language,
                    "groups": model.groups,
                    "create_at": create_at,
                    "update_at": update_at,
                }
            }))
        }
        Err(e) => {
            let msg = format!("Error during update: {:?}", e);
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": msg
                }))
        }
    }
}