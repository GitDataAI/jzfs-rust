use actix_web::web;

pub mod team;

pub fn org(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/team")
                .configure(team::team)
        )
    ;
}