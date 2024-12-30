use crate::avatar::download::avatar_download;
use crate::avatar::upload::avatar_upload;
use actix_web::web;

pub mod download;
pub mod upload;

pub fn avatar(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/{uid}", web::put().to(avatar_upload))
        .service(
            web::resource("/{uid}")
                .get(avatar_download)
        )
    ;
}
