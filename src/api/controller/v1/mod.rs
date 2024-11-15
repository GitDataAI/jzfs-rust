use actix_web::web;

pub mod auth;
pub mod keys;

#[inline]
pub fn v1_router(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("auth")
                .configure(auth::auth)
        )
        .service(
            web::scope("/keys")
                .configure(keys::keys)
        )
    ;
}