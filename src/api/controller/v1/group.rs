use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{post, Json, ServiceConfig};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use crate::api::controller::session::UsersModel;
use crate::api::server::group::GroupServer;
use crate::db::model::users;

#[derive(Deserialize)]
pub struct CreateGroup{
    pub name:  String,
    pub bio: String,
    pub contact: String,
}
#[derive(Deserialize)]
pub struct GroupUid{
    pub uid: Uuid,
}

#[derive(Deserialize)]
pub struct GroupReName{
    pub uid: Uuid,
    pub group_name: String,
    pub new_name: String,
}
#[derive(Deserialize)]
pub struct GroupReBio{
    pub uid: Uuid,
    pub bio: String,
}
#[derive(Deserialize)]
pub struct GroupReLocation {
    pub uid: Uuid,
    pub location: String,
}
#[derive(Deserialize)]
pub struct GroupUsers{
    pub uid: Uuid,
    pub user_id: Uuid,
}

#[inline]
pub fn groups(cfg: &mut ServiceConfig){
    cfg
        .route("create",post().to(create_group))
        .route("getid",post().to(get_by_uid))
        .route("rename",post().to(rename))
        .route("rebio",post().to(rebio))
        .route("relocation",post().to(relocation))
        .route("auser",post().to(add_user))
        .route("ruser",post().to(remove_user))
    ;
}


#[inline]
pub async fn create_group(session: Session,dto: Json<CreateGroup>) -> impl Responder{
    let group = GroupServer::new();
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    match group.create_group(dto.name.clone(),dto.bio.clone(),model.uid,dto.contact.clone()).await{
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "msg": "success",
                    "data": data
                }))
        }
        Err(error) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": error.to_string(),
                }))
        }
    }
}
#[inline]
pub async fn get_by_uid(dto: Json<GroupUid>) -> impl Responder{
    let group = GroupServer::new();
    match group.get_from_uid(dto.uid.clone()).await {
        Ok(data) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "msg": "success",
                    "data": data
                }))
        }
        Err(error) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": error.to_string(),
                }))
        }
    }
}
#[inline]
pub async fn rename(session: Session, dto: Json<GroupReName>) -> impl Responder{
    let group = GroupServer::new();
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    match group.rename(dto.uid,dto.group_name.clone(),dto.new_name.clone(),model.uid.clone()).await {
        Ok(data) => {
            let data = data.rows_affected;
            HttpResponse::Ok()
            .json(json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "rows_affected": data
                }
            }))
        },
        Err(error) => {
            let err = format!("{:?}", error);
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": err.to_string()
                }))
        }
    }
}
#[inline]
pub async fn rebio(session: Session, dto: Json<GroupReBio>) -> impl Responder{
    let group = GroupServer::new();
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    match group.rebio(dto.uid,dto.bio.clone(),model.uid.clone()).await {
        Ok(data) => {
            let data = data.rows_affected;
            HttpResponse::Ok()
                .json(json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "rows_affected": data
                }
            }))
        },
        Err(error) => {
            let err = format!("{:?}", error);
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": err.to_string()
                }))
        }
    }
}
#[inline]
pub async fn relocation(session: Session,dto: Json<GroupReLocation>) -> impl Responder{
    let group = GroupServer::new();
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    match group.relocation(dto.uid,dto.location.clone(),model.uid.clone()).await {
        Ok(data) => {
            let data = data.rows_affected;
            HttpResponse::Ok()
                .json(json!({
                "code": 200,
                "msg": "success",
                "data": {
                    "rows_affected": data
                }
            }))
        },
        Err(error) => {
            let err = format!("{:?}", error);
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": err.to_string()
                }))
        }
    }
}
#[inline]
pub async fn add_user(session: Session, dto: Json<GroupUsers>) -> impl Responder{
    let group = GroupServer::new();
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    match group.add_user(dto.uid,dto.user_id,model.uid).await{
        Ok(_) => {
            HttpResponse::Ok()
                .json(json!({
                     "code": 200,
                     "msg": "success",
                }))
        }
        Err(e) => {
            let err = format!("{:?}", e);
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": err.to_string()
                }))
        }
    }
}
#[inline]
pub async fn remove_user(session: Session, dto: Json<GroupUsers>) -> impl Responder{
    let group = GroupServer::new();
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    match group.remove_user(dto.uid,dto.user_id,model.uid).await {
        Ok(_) => {
            HttpResponse::Ok()
            .json(json!({
                "code": 200,
                "msg": "success",
            }))
        }
        Err(e) => {
            let err = format!("{:?}", e);
            HttpResponse::Ok()
            .json(json!({
                "code": 500,
                "msg": err.to_string()
            }))
        }
    }
}