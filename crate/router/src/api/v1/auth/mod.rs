mod login;
mod logout;
mod forget;
mod local;
use actix_web::web;

pub fn auth(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/login")
                .route("/username",web::post().to(login::auth_by_username_password))
                .route("/email",web::post().to(login::auth_by_email_password))
        )
        .route("/logout",web::post().to(logout::logout))
        .route("/local", web::post().to(local::local))
    ;
}