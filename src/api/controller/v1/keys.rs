use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Json;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use crate::api::controller::session::UsersModel;
use crate::api::server::keys::KeysServer;
use crate::db::model::users;

#[derive(Deserialize)]
pub struct Keys{
    name: String,
    public_key: Option<String>,
}
#[derive(Deserialize)]
pub struct Token{
    token: String,
}
#[derive(Deserialize)]
pub struct PubKey{
    public_key: String,
}
#[inline]
pub fn keys(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("list")
                .route("token",web::post().to(list_token))
                .route("pubkey",web::post().to(list_public_key))
        )
        .service(
            web::scope("add")
                .route("token",web::post().to(generate_token))
                .route("pubkey", web::post().to(add_public_key))
        )
        .service(
            web::scope("remove")
                .route("token",web::post().to(del_token))
                .route("pubkey", web::post().to(del_public_key))
        )
    ;
}
#[inline]
pub async fn generate_token(session: Session,key: Json<Keys>) -> impl Responder{
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
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(96)
        .map(char::from)
        .collect();
    let keys = KeysServer::new();
    match keys.add_token(model.uid, token, key.name.clone()).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "data": data
                }))
        }
        Err(err) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}
#[inline]
pub async fn add_public_key(session: Session,key: Json<Keys>) -> impl Responder{
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
    if key.public_key.is_none() {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
            "code": 403,
            "msg": "Public Key NotFound"
        }))
    }
    let public_key = key.public_key.clone().unwrap();

    let keys = KeysServer::new();
    match keys.add_public_key(model.uid, public_key, key.name.clone()).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "data": data
                }))
        }
        Err(err) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}
#[inline]
pub async fn list_public_key(session: Session) -> impl Responder{
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


    let keys = KeysServer::new();
    match keys.list_public_keys(model.uid).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "data": data
                }))
        }
        Err(err) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}
#[inline]
pub async fn list_token(session: Session) -> impl Responder{
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


    let keys = KeysServer::new();
    match keys.list_token(model.uid).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "data": data
                }))
        }
        Err(err) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}
#[inline]
pub async fn del_public_key(session: Session, pub_key: Json<PubKey>) -> impl Responder{
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
    let keys = KeysServer::new();
    match keys.del_public_key(model.uid,pub_key.public_key.clone()).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "data": {
                        "rows_affected": data.rows_affected
                    }
                }))
        }
        Err(err) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}
#[inline]
pub async fn del_token(session: Session, token: Json<Token>) -> impl Responder{
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
    let keys = KeysServer::new();
    match keys.del_public_token(model.uid,token.token.clone()).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "data": {
                        "rows_affected": data.rows_affected
                    }
                }))
        }
        Err(err) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}
