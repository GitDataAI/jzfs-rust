use actix_web::middleware::from_fn;
use actix_web::web;
use crate::api::handler::email::captcha::{api_email_captcha_check, api_email_rand_captcha};
use crate::api::handler::email::forget::api_email_forget;
use crate::api::handler::group::check::{api_group_check_name, api_group_owner_check};
use crate::api::handler::group::create::api_group_create;
use crate::api::handler::group::delete::api_group_delete;
use crate::api::handler::group::list::api_group_list_owner;
use crate::api::handler::group::search::api_group_search_name;
use crate::api::handler::users::apply::api_user_apply;
use crate::api::handler::users::local::api_user_local;
use crate::api::handler::users::login::{api_users_login_email, api_users_login_name};
use crate::api::handler::users::logout::api_user_logout;
use crate::api::handler::users::reset::{api_user_reset_passwd_forget, api_user_reset_passwd_profile};
use crate::api::handler::users::update::api_user_update;
use crate::api::handler::version::api_version;
use crate::api::middleware::auth::must_allow_next::must_login;

pub fn endpoint(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/api")
                .route("/version",web::get().to(api_version))
                .service(
                    web::scope("/v1")
                        .service(
                            web::scope("/users")
                                .route("/apply", web::post().to(api_user_apply))
                                .route("/login/name",web::post().to(api_users_login_name))
                                .route("/login/email",web::post().to(api_users_login_email))
                                .route("/reset/online",web::post().to(api_user_reset_passwd_profile))
                                .route("/reset/forget",web::post().to(api_user_reset_passwd_forget))
                                .service(
                                    web::scope("/logout")
                                        .wrap(from_fn(must_login))
                                        .default_service(web::post().to(api_user_logout))
                                )
                                .service(
                                    web::scope("/update")
                                        .wrap(from_fn(must_login))
                                        .default_service(web::post().to(api_user_update))
                                )
                                .service(
                                    web::scope("/local")
                                        .wrap(from_fn(must_login))
                                        .default_service(web::post().to(api_user_local))
                                )
                        )                   
                        .service(
                            web::scope("/email")
                                .service(
                                    web::scope("/captcha")
                                        .route("/send", web::post().to(api_email_rand_captcha))
                                        .route("/verify", web::post().to(api_email_captcha_check))
                                )
                                .route("/forget",web::post().to(api_email_forget))
                        )
                        .service(
                            web::scope("/group")
                                .wrap(from_fn(must_login))
                                .route("/create", web::post().to(api_group_create))
                                .route("/delete", web::post().to(api_group_delete))
                                .service(
                                    web::scope("/list")
                                        .route("/owner",web::post().to(api_group_list_owner))
                                )
                                .service(
                                    web::scope("/search")
                                        .route("/name",web::post().to(api_group_search_name))
                                )
                                .service(
                                    web::scope("/check")
                                        .route("/name", web::post().to(api_group_check_name))
                                        .route("/owner", web::post().to(api_group_owner_check))
                                )
                        )
                )
        )
    ;
}