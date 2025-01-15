use actix_identity::Identity;
use actix_session::Session;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::web;

use crate::apis::app_writer::AppWrite;
use crate::service::AppState;
use crate::service::auth::AuthInner;
use crate::service::auth::passwd::UsersAuthPasswdParam;

/// /api/v1/auth/passwd POST
pub async fn api_v1_auth_passwd(
    session : Session,
    param : web::Json<AuthInner>,
    request : HttpRequest,
    app_state : web::Data<AppState>,
) -> impl Responder {
    let inner = match param.decode::<UsersAuthPasswdParam>() {
        Ok(inner) => inner,
        Err(err) => {
            return AppWrite::fail(err.to_string());
        }
    };
    match app_state.auth_by_passwd(inner).await {
        Ok(token) => {
            session.insert("token", token.clone()).unwrap();
            Identity::login(
                &request.extensions(),
                serde_json::to_string(&token).unwrap(),
            )
            .unwrap();
            AppWrite::ok(token)
        }
        Err(err) => AppWrite::fail(err.to_string()),
    }
}
