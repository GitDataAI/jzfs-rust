use actix_session::Session;
use actix_web::Responder;
use actix_web::web;
use gitdata::model::repository::repository;

use crate::apis::app_writer::AppWrite;
use crate::service::AppState;
use crate::service::users::info::UsersInfoReplay;

pub async fn api_v1_repo_info(
    state : web::Data<AppState>,
    param : web::Path<(String, String)>,
    session : Session,
) -> impl Responder {
    let repo = match state
        .repo_owner_name(param.0.clone(), param.1.clone())
        .await
    {
        Ok(repo) => repo,
        Err(_) => {
            return AppWrite::<repository::Model>::not_found("Repository Not Found".to_string());
        }
    };
    if repo.visible {
        return AppWrite::ok(repo);
    } else {
        if let Ok(user) = UsersInfoReplay::from_session(session) {
            if user.uid == repo.owner_uid {
                return AppWrite::ok(repo);
            } else {
                if user.member.contains(&user.uid) {
                    return AppWrite::ok(repo);
                }
            }
        }
    }
    AppWrite::<repository::Model>::not_found("Repository Not Found".to_string())
}
