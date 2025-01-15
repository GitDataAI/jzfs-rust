use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::users::info::UsersInfoReplay;
use crate::service::users::setting::UsersPinedParam;
use crate::service::users::setting::UsersSettingAvatarParam;
use crate::service::users::setting::UsersSettingBasicParam;
use crate::service::users::setting::UsersSettingTopicParam;

/// /api/v1/user/setting/basic POST
pub async fn api_v1_user_setting_basic(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<UsersSettingBasicParam>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<String>::fail(err.to_string());
        }
    };
    match app_state
        .users_setting_basic_update(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/user/setting/topic POST
pub async fn api_v1_user_setting_topic(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<UsersSettingTopicParam>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<String>::fail(err.to_string());
        }
    };
    match app_state
        .users_setting_topic_update(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/user/setting/avatar POST
pub async fn api_v1_user_setting_avatar(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<UsersSettingAvatarParam>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<String>::fail(err.to_string());
        }
    };
    match app_state
        .users_setting_avatar_update(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
/// /api/v1/user/setting/pinned POST
pub async fn api_v1_user_setting_pinned(
    session : Session,
    app_state : web::Data<crate::service::AppState>,
    param : web::Json<UsersPinedParam>,
) -> impl Responder {
    let ident = match UsersInfoReplay::from_session(session) {
        Ok(ident) => ident,
        Err(err) => {
            return AppWrite::<String>::fail(err.to_string());
        }
    };
    match app_state
        .users_setting_pined_update(ident.uid, param.into_inner())
        .await
    {
        Ok(_) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
