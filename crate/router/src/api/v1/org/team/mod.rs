use actix_web::web;

pub mod create;
pub mod update;
pub mod delete;
pub mod get;
pub fn team(cfg: &mut web::ServiceConfig){
    cfg
        .route("/create", web::post().to(create::team_create))
        .route("/update", web::post().to(update::team_update))
        .route("/delete", web::post().to(delete::team_delete))
        .service(
            web::scope("/get")
                .route("/name", web::post().to(get::team_get_by_name))
                .route("/id", web::post().to(get::team_get_by_uid))
        )
    ;
}