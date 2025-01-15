use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::users::info::UsersInfoReplay;
use crate::service::users::token::TokenCreateParam;
use crate::service::users::token::TokenCreateReply;
use crate::service::users::token::TokenDeleteParam;
use crate::service::users::token::TokenListParam;
use crate::service::users::token::TokenListReply;

/// /api/v1/user/token POST
pub async fn api_v1_user_token_create(
    session : Session,
    param : web::Json<TokenCreateParam>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => return AppWrite::<TokenCreateReply>::fail(err.to_string()),
    };
    match app_state
        .users_generate_token(ident.uid, param.into_inner())
        .await
    {
        Ok(token) => AppWrite::ok(token),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/user/token DELETE
pub async fn api_v1_user_token_delete(
    session : Session,
    param : web::Json<TokenDeleteParam>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => return AppWrite::<String>::fail(err.to_string()),
    };
    match app_state
        .users_token_delete(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/user/token GET
pub async fn api_v1_user_token_list(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<TokenListParam>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => return AppWrite::<Vec<TokenListReply>>::fail(err.to_string()),
    };
    match app_state
        .users_token_list(ident.uid, param.into_inner())
        .await
    {
        Ok(tokens) => AppWrite::ok(tokens),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
