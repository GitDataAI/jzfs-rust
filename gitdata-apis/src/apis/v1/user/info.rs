use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::users::info::UsersInfoReplay;

/// /user/info GET
pub async fn api_v1_users_info(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<UsersInfoReplay>::fail(err.to_string());
        }
    };
    match app_state.users_info_by_uid(ident.uid).await {
        Ok(info) => AppWrite::ok(info),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /user/info/{username} GET
pub async fn api_v1_users_info_by_username(
    param : web::Path<String>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    match app_state.users_info_by_username(param.into_inner()).await {
        Ok(info) => AppWrite::ok(info),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
