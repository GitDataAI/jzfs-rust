use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::auth::AuthInner;
use crate::service::users::apply::UsersApplyParam;

///  /api/v1/user/apply POST
pub async fn api_v1_user_apply(
    param : web::Json<AuthInner>,
    app_state : web::Data<crate::service::AppState>,
) -> impl Responder {
    let param = match param.decode::<UsersApplyParam>() {
        Ok(param) => param,
        Err(err) => {
            return AppWrite::fail(err.to_string());
        }
    };
    match app_state.users_apply(param).await {
        Ok(_token) => AppWrite::ok("".to_string()),
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
