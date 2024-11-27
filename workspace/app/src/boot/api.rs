use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{web, App, HttpServer};
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::middleware::Logger;
use config::redis::Redis;
use crate::router;
use crate::service::Service;

pub async fn run() -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    log::info!("Starting server");
    let service = Service::new().await;
    let redis = Redis::init().await;
    let session = RedisSessionStore::builder_pooled(redis.clone().pool).build().await?;
    let start = std::time::Instant::now();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis.clone()))
            .app_data(web::Data::new(service.clone()))
            .app_data(web::Data::new(start))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    session.clone(),
                    Key::from(&[0; 64])
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
            .configure(router::api::endpoint)
    })
        .bind(("0.0.0.0", 80))?
        .run()
        .await?;
    Ok(())
}