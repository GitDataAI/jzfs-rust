use actix_web::web;

pub mod captcha;


pub fn email(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/captcha")
                .route("/send",web::post().to(captcha::send))
                .route("/check",web::post().to(captcha::check))
        )
    ;
}