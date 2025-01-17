use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::AppState;
use crate::service::repository::repo::RepoCreateParam;
use crate::service::repository::repo::RepoReNameParma;
use crate::service::repository::repo::RepoVisibleParma;
use crate::service::users::info::UsersInfoReplay;

pub async fn api_v1_repo_create(
    state : web::Data<AppState>,
    param : web::Json<RepoCreateParam>,
    session : Session,
) -> impl Responder {
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
    };
    if param.owner_uid == user.uid {
        let user_info = match state._users_info_by_uid(user.uid).await {
            Ok(user_info) => user_info,
            Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
        };
        let state = state.repository_new(user_info, param.into_inner()).await;
        match state {
            Ok(_) => AppWrite::ok("Repository created".to_string()),
            Err(e) => {
                dbg!(e);
                AppWrite::internal_server_error("Internal server error".to_string())
            }
        }
    } else {
        let origin = match state._users_info_by_uid(param.owner_uid).await {
            Ok(origin) => origin,
            Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
        };
        if origin.member.contains(&user.uid) {
            let state = state.repository_new(origin, param.into_inner()).await;
            match state {
                Ok(_) => AppWrite::ok("Repository created".to_string()),
                Err(e) => {
                    dbg!(e);
                    AppWrite::internal_server_error("Internal server error".to_string())
                }
            }
        } else {
            AppWrite::unauthorized("Unauthorized".to_string())
        }
    }
}

pub async fn api_v1_repo_remove(
    state : web::Data<AppState>,
    param : web::Path<(String, String)>,
    session : Session,
) -> impl Responder {
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
    };
    let repo = match state
        .repo_owner_name(param.0.clone(), param.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::not_found("Repository not found".to_string()),
    };
    if repo.owner_uid == user.uid {
        let state = state.repository_delete(repo).await;
        match state {
            Ok(_) => AppWrite::ok("Repository deleted".to_string()),
            Err(_) => AppWrite::internal_server_error("Internal server error".to_string()),
        }
    } else {
        AppWrite::unauthorized("Unauthorized".to_string())
    }
}

pub async fn api_v1_repo_rename(
    state : web::Data<AppState>,
    query : web::Path<(String, String)>,
    parma : web::Json<RepoReNameParma>,
    session : Session,
) -> impl Responder {
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
    };
    let repo = match state
        .repo_owner_name(query.0.clone(), query.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::not_found("Repository not found".to_string()),
    };
    if repo.owner_uid == user.uid {
        let state = state.repository_rename(repo, parma.into_inner()).await;
        match state {
            Ok(_) => AppWrite::ok("Repository renamed".to_string()),
            Err(_) => AppWrite::internal_server_error("Internal server error".to_string()),
        }
    } else {
        let user = match state._users_info_by_uid(repo.owner_uid).await {
            Ok(user) => user,
            Err(_) => return AppWrite::not_found("Repository not found".to_string()),
        };
        if user.member.contains(&user.uid) {
            let state = state.repository_rename(repo, parma.into_inner()).await;
            match state {
                Ok(_) => AppWrite::ok("Repository renamed".to_string()),
                Err(_) => AppWrite::internal_server_error("Internal server error".to_string()),
            }
        } else {
            AppWrite::unauthorized("Unauthorized".to_string())
        }
    }
}

pub async fn api_v1_repo_visible(
    state : web::Data<AppState>,
    query : web::Path<(String, String)>,
    parma : web::Json<RepoVisibleParma>,
    session : Session,
) -> impl Responder {
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
    };
    let repo = match state
        .repo_owner_name(query.0.clone(), query.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => return AppWrite::not_found("Repository not found".to_string()),
    };
    if repo.owner_uid == user.uid {
        let state = state.repository_visible(repo, parma.into_inner()).await;
        match state {
            Ok(_) => AppWrite::ok("Repository visible".to_string()),
            Err(_) => AppWrite::internal_server_error("Internal server error".to_string()),
        }
    } else {
        let user = match state._users_info_by_uid(repo.owner_uid).await {
            Ok(user) => user,
            Err(_) => return AppWrite::not_found("Repository not found".to_string()),
        };
        if user.member.contains(&user.uid) {
            let state = state.repository_visible(repo, parma.into_inner()).await;
            match state {
                Ok(_) => AppWrite::ok("Repository visible".to_string()),
                Err(_) => AppWrite::internal_server_error("Internal server error".to_string()),
            }
        } else {
            AppWrite::unauthorized("Unauthorized".to_string())
        }
    }
}

pub async fn api_v1_repo_crw(session : Session, state : web::Data<AppState>) -> impl Responder {
    let user = match UsersInfoReplay::from_session(session) {
        Ok(user) => user,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string()),
    };
    let state = state.repository_create_owner(user.uid).await;
    match state {
        Ok(repos) => AppWrite::ok(repos),
        Err(_) => AppWrite::internal_server_error("Internal server error".to_string()),
    }
}
