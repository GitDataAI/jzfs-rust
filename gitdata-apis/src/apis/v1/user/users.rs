use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::AppState;
use crate::service::users::info::UsersInfoReplay;

pub async fn api_v1_users_repo(
    path : web::Path<String>,
    state : web::Data<AppState>,
) -> impl Responder {
    let user = match state._users_info_by_username(path.into_inner()).await {
        Ok(user) => user,
        Err(_) => return AppWrite::not_found("User not found".to_string()),
    };
    let repo = match state.repository_owner(user.uid).await {
        Ok(repo) => repo,
        Err(_) => return AppWrite::not_found("Repository not found".to_string()),
    };
    AppWrite::ok(repo)
}

pub async fn api_v1_users_repo_session(
    session : Session,
    state : web::Data<AppState>,
) -> impl Responder {
    let users = match UsersInfoReplay::from_session(session) {
        Ok(users) => users,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
    };
    let repo = match state.repository_owner(users.uid).await {
        Ok(repo) => repo,
        Err(_) => return AppWrite::not_found("Repository not found".to_string()),
    };
    AppWrite::ok(repo)
}
