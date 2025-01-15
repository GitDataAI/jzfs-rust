use actix_session::Session;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::emails::captcha::EmailCaptchaCheckParam;
use crate::service::emails::captcha::EmailCaptchaParam;

/// /api/v1/email/captcha POST
pub async fn api_v1_email_captcha_post(
    session : Session,
    param : web::Json<EmailCaptchaParam>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    match app_state
        .email_captcha_send(param.into_inner(), session)
        .await
    {
        Ok(_) => AppWrite::<String>::ok("Send success".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}

/// /api/v1/email/captcha/check POST
pub async fn api_v1_email_captcha_check(
    session : Session,
    param : web::Json<EmailCaptchaCheckParam>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    match app_state
        .email_captcha_check(param.into_inner(), session)
        .await
    {
        Ok(result) => AppWrite::<bool>::ok(result),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
