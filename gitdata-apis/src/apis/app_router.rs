use actix_web::web::post;
use actix_web::web::scope;

use super::v1::auth::logout::api_v1_auth_logout;
use super::v1::auth::passwd::api_v1_auth_passwd;
use super::v1::email::captcha::api_v1_email_captcha_check;
use super::v1::email::captcha::api_v1_email_captcha_post;

#[allow(non_snake_case)]
pub async fn AppRouter(cfg : &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api").service(
            scope("/v1")
                .service(
                    scope("/auth")
                        .route("/passwd", post().to(api_v1_auth_passwd))
                        .route("/logout", post().to(api_v1_auth_logout)),
                )
                .service(
                    scope("/email")
                        .route("/captcha", post().to(api_v1_email_captcha_post))
                        .route("/captcha/check", post().to(api_v1_email_captcha_check)),
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
                            ),
                    ),
                ),
        ),
    );
}
