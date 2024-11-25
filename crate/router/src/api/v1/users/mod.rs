use actix_web::web;

pub mod apply;
pub mod get;
pub fn users(cfg: &mut web::ServiceConfig){
    cfg
        .route("/apply", web::post().to(apply::apply))
        .service(
            web::scope("/get")
                .route("/name", web::post().to(get::get_user_by_name))
                .route("/id", web::post().to(get::get_user_by_uid))
        )
    ;
}