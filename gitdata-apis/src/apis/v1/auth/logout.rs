use actix_identity::Identity;
use actix_session::Session;
use actix_web::Responder;

use crate::apis::app_writer::AppWrite;

/// /api/v1/auth/logout POST
pub async fn api_v1_auth_logout(session : Session, identity : Identity) -> impl Responder {
    identity.logout();
    session.purge();
    AppWrite::<String>::ok("Logout success".to_string())
}
