use crate::api::v1::v1;
use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use jzfs_module::entity::config::anyhow;
use jzfs_module::service::redis::Redis;
use jzfs_module::Module;
use crate::api::v1::version::version;

pub async fn run() -> anyhow::Result<()>{
    let module = Module::init().await;
    let redis = Redis::init();
    let session = RedisSessionStore::builder_pooled(redis.clone().pool).build().await?;
    HttpServer::new(move || {
        let key = Key::from(&[0; 64]);
        App::new()
            .app_data(web::Data::new(redis.clone()))
            .app_data(web::Data::new(module.clone()))
            .wrap(
                SessionMiddleware::builder(
                    session.clone(),
                    key.clone()
                )
                    .cookie_name("SessionID".to_string())
                    .cookie_path("/".to_string())
                    .cookie_http_only(false)
                    .cookie_content_security(
                        CookieContentSecurity::Private
                    )
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::days(30))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnStateChanges)
                    )
                    .cookie_secure(false)
                    .build()
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/v1")
                            .configure(v1)
                    )
                    .route("/version",web::get().to(version))
            )
            
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
