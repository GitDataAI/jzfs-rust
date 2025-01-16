use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::users::info::UsersInfoReplay;
use crate::service::users::ssh_keys::SShKeysListReply;
use crate::service::users::ssh_keys::SshKeyDeleteParam;
use crate::service::users::ssh_keys::SshKeysCreateParam;

/// /api/v1/user/ssh POST
pub async fn api_v1_user_ssh_key_create(
    session : Session,
    param : web::Json<SshKeysCreateParam>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<String>::fail(err.to_string());
        }
    };

    match app_state
        .users_ssh_key_create(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/user/ssh DELETE
pub async fn api_v1_user_ssh_key_delete(
    session : Session,
    param : web::Query<SshKeyDeleteParam>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<String>::fail(err.to_string());
        }
    };

    match app_state
        .users_ssh_key_delete(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/user/ssh GET
pub async fn api_v1_user_ssh_key_list(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<Vec<SShKeysListReply>>::fail(err.to_string());
        }
    };

    match app_state.users_ssh_key_list(ident.uid).await {
        Ok(keys) => AppWrite::ok(keys),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
