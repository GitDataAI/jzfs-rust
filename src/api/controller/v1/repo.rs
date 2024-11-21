use crate::api::controller::session::UsersModel;
use crate::api::server::repo::RepoServer;
use crate::db::model::repo::RepoOrigin;
use crate::db::model::{repo, users};
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RepoCreate{
    name: String,
    repo_avatar_url: Option<String>,
    visible: bool,
    bio: String,
    origin: Uuid,
    is_group: bool,
}
#[derive(Deserialize)]
pub struct RepoVisible{
    uid: Uuid,
    owner_id: Uuid,
    is_group: bool,
    visible: bool,
}
#[derive(Deserialize)]
pub struct RepoBio{
    uid: Uuid,
    owner_id: Uuid,
    is_group: bool,
    bio: String,
}

pub fn repo(cfg: &mut web::ServiceConfig) {
    cfg
        .route("owner", web::post().to(owner))

    ;
}

pub async fn owner(session: Session) -> impl Responder {
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    let repo = RepoServer::new();
    match repo.get_owner_repo(RepoOrigin::User(model.uid)).await{
        Ok(data) => {
            HttpResponse::Ok().json(json!({
                "code": 200,
                "msg": "Success",
                "data": data
            }))
        }
        Err(e) => {
            let err = format!("Internal server error: {:?}", e);
            HttpResponse::Ok().json(json!({
                "code": 500,
                "msg": err,
            }))
        }
    }
}
pub async fn create_repo(session: Session,repo: Json<RepoCreate>) -> impl Responder {
    let model = session.get::<users::Model>(UsersModel).unwrap().unwrap();
    let repos = RepoServer::new();
    let model = repo::ActiveModel{
        uid: Default::default(),
        name: Set(repo.name.clone()),
        repo_avatar_url: Set(repo.repo_avatar_url.clone()),
        origin: Set(match repo.is_group {
            true => {
                serde_json::to_string(&RepoOrigin::Group(repo.origin)).unwrap().into()
            }
            false => {
                serde_json::to_string(&RepoOrigin::User(repo.origin)).unwrap().into()
            }
        }),
        visible: Set(repo.visible),
        use_storage: Default::default(),
        bio: Set(repo.bio.clone()),
        branch: Set(Vec::new()),
        forks: Set(0),
        stars: Set(0),
        fork_from: Set(None),
        create_id: Set(model.uid),
        create_at: Default::default(),
        update_at: Default::default(),
    };
    match repos.create_repo(model).await{
        Ok(_) => {
            HttpResponse::Ok().json(json!({"code": 200, "msg": "Success"}))
        }
        Err(e) => {
            let err = format!("Internal server error: {}", e);
            HttpResponse::Ok().json(json!({"code": 500,"msg": err,}))
        }
    }
}
pub async fn update_visible(dto: Json<RepoVisible>) -> impl Responder {
    let repo = RepoServer::new();
    match repo.update_visible(dto.uid,{
        match dto.is_group {
            true => {
                RepoOrigin::Group(dto.owner_id)
            }
            false => {
                RepoOrigin::User(dto.owner_id)
            }
        }
    }, dto.visible).await{
        Ok(result) => {
            let result = format!("{}",result.rows_affected);
            HttpResponse::Ok().json(json!({
                "code": 200,
                "msg": "Success",
                "data": {
                    "rows_affected": result
                }
            }))
        }
        Err(e) => {
            let err = format!("Internal server error: {}", e);
            HttpResponse::Ok().json(json!({
                "code": 500,
                "msg": err,
            }))
        }
    }
}
pub async fn update_bio(dto: Json<RepoBio>) -> impl Responder {
    let repo = RepoServer::new();
    match repo.update_bio(dto.uid,{
        match dto.is_group {
            true => {
                RepoOrigin::Group(dto.owner_id)
            }
            false => {
                RepoOrigin::User(dto.owner_id)
            }
        }
    }, dto.bio.clone()).await{
        Ok(result) => {
            let result = format!("{}",result.rows_affected);
            HttpResponse::Ok().json(json!({
                "code": 200,
                "msg": "Success",
                "data": {
                    "rows_affected": result
                }
            }))
        }
        Err(e) => {
            let err = format!("Internal server error: {}", e);
            HttpResponse::Ok().json(json!({
                "code": 500,
                "msg": err,
            }))
        }
    }
}

// pub async fn move_repo(dto: Json<RepoMove>) -> impl Responder{
//
// }