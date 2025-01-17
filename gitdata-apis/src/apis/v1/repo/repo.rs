use actix_session::Session;
use actix_web::{web, Responder};
use crate::apis::app_writer::AppWrite;
use crate::service::AppState;
use crate::service::repository::new_repo::RepoCreateParam;
use crate::service::users::info::UsersInfoReplay;

pub async fn api_v1_repo_create(
    state : web::Data<AppState>,
    param : web::Json<RepoCreateParam>,
    session: Session
) -> impl Responder
{
    let user = match UsersInfoReplay::from_session(session){
        Ok(user) => user,
        Err(_) => return AppWrite::unauthorized("Unauthorized".to_string())
    };
    if param.owner_uid == user.uid {
        let user_info = match state._users_info_by_uid(user.uid).await {
            Ok(user_info) => user_info,
            Err(_) => return AppWrite::unauthorized("Unauthorized".to_string())
        };
        let state = state.repository_new(user_info, param.into_inner()).await;
        match state {
            Ok(_) => AppWrite::ok("Repository created".to_string()),
            Err(_) => AppWrite::internal_server_error("Internal server error".to_string())
        }
    }else { 
        let origin = match state._users_info_by_uid(param.owner_uid).await {
            Ok(origin) => origin,
            Err(_) => return AppWrite::unauthorized("Unauthorized".to_string())
        };
        if origin.member.contains(&user.uid){
            let state = state.repository_new(origin, param.into_inner()).await;
            match state {
                Ok(_) => AppWrite::ok("Repository created".to_string()),
                Err(_) => AppWrite::internal_server_error("Internal server error".to_string())
            }
        }else { 
            AppWrite::unauthorized("Unauthorized".to_string())
        }
    }
}
