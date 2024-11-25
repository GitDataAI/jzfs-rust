use actix_web::web;

pub mod users;
pub mod auth;
pub mod repo;
pub mod org;
pub mod object;
pub mod notify;
pub mod misc;
pub mod version;
pub mod setup;
mod email;

pub fn v1(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/auth")
                .configure(auth::auth)
        )
        .service(
            web::scope("/email")
                .configure(email::email)
        )
        .service(
            web::scope("/org")
                .configure(org::org)
        )
    ;
}