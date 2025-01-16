use actix_web::web::{delete, get, post};
use actix_web::web::scope;

#[allow(non_snake_case)]
pub fn AppRouter(cfg : &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api").service(
            scope("/v1")
                .service(
                    scope("/auth")
                        .route("/passwd", post().to(crate::apis::v1::auth::passwd::api_v1_auth_passwd))
                        .route("/logout", post().to(crate::apis::v1::auth::logout::api_v1_auth_logout)),
                )
                .service(
                    scope("/email")
                        .route("/captcha", post().to(crate::apis::v1::email::captcha::api_v1_email_captcha_post))
                        .route("/captcha/check", post().to(crate::apis::v1::email::captcha::api_v1_email_captcha_check)),
                )
                .service(
                    scope("/user").service(
                        scope("/setting")
                            .route(
                                "/basic",
                                post().to(super::v1::user::setting::api_v1_user_setting_basic),
                            )
                            .route(
                                "/topic",
                                post().to(super::v1::user::setting::api_v1_user_setting_topic),
                            )
                            .route(
                                "/avatar",
                                post().to(super::v1::user::setting::api_v1_user_setting_avatar),
                            )
                            .route(
                                "/pined",
                                post().to(super::v1::user::setting::api_v1_user_setting_pinned),
                            )
                        ,
                    ).route("/apply", post().to(super::v1::user::apply::api_v1_user_apply))
                        .service(
                            scope("/info")
                                .route("", get().to(super::v1::user::info::api_v1_users_info))
                                .route(
                                    "/{username}",
                                    get().to(super::v1::user::info::api_v1_users_info_by_username),
                                )
                        )
                        .service(
                            scope("/ssh")
                                .route("", get().to(super::v1::user::ssh_keys::api_v1_user_ssh_key_list))
                                .route("/{ssh_key_uid}", delete().to(super::v1::user::ssh_keys::api_v1_user_ssh_key_delete))
                                .route("", post().to(super::v1::user::ssh_keys::api_v1_user_ssh_key_create))
                        )
                        .service(
                            scope("token")
                                .route("", post().to(super::v1::user::token_key::api_v1_user_token_create))
                                .route("/{token_uid}", delete().to(super::v1::user::token_key::api_v1_user_token_delete))
                                .route("", get().to(super::v1::user::token_key::api_v1_user_token_list))
                        )
                    ,
                ),
        ),
    );
}
