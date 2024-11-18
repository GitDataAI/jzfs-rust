use actix_web::middleware::from_fn;
use actix_web::web;
use crate::api::middleware::session_guard::session_guard_handler;

pub mod auth;
pub mod keys;
pub mod repo;
mod group;

#[inline]
pub fn v1_router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("auth")
                .configure(auth::auth)
        )
        .service(
            web::scope("keys")
                .wrap(from_fn(session_guard_handler))
                .configure(keys::keys)
        )
        .service(
            web::scope("repo")
                .wrap(from_fn(session_guard_handler))
                .configure(repo::repo)
        )
        .service(
            web::scope("group")
                .wrap(from_fn(session_guard_handler))
                .configure(group::groups)
        )
    ;
}