use actix_session::SessionMiddleware;
use actix_session::config::CookieContentSecurity;
use actix_session::config::PersistentSession;
use actix_session::config::TtlExtensionPolicy;
use actix_session::storage::RedisSessionStore;
use actix_settings::ApplySettings;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use actix_web::web;
use gitdata_apis::apis::app_router::AppRouter;
use gitdata_apis::service::AppState;
use tracing::info;

const CONFIG_FILE : &str = "./config/api.toml";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    info!("Starting server");
    info!("Loading Api configuration");
    let api_config = if let Ok(api_config) = actix_settings::Settings::parse_toml(CONFIG_FILE) {
        api_config
    } else {
        actix_settings::Settings::write_toml_file(CONFIG_FILE)?;
        return Err(anyhow::anyhow!("Please configure the configuration file"));
    };
    let state = AppState::init().await?;
    let session = RedisSessionStore::builder_pooled(state.redis_pool.clone())
        .build()
        .await?;
    info!("Redis session store initialized.");
    info!("API server started.");
    HttpServer::new(move || {
        App::new()
            .wrap(actix_identity::IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(session.clone(), Key::from(&[0; 64]))
                    .cookie_name("SessionID".to_string())
                    .cookie_path("/".to_string())
                    .cookie_http_only(false)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::days(30))
                            .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                    )
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .configure(AppRouter)
    })
    .try_apply_settings(&api_config)
    .expect("Failed to apply settings")
    .run()
    .await
    .expect("Failed to start server");
    Ok(())
}
