pub mod v1;
pub(crate) mod session;

use crate::api::controller::v1::v1_router;
use crate::config::CFG;
use actix_cors::Cors;
use actix_session::config::{PersistentSession, TtlExtensionPolicy};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::dev::Service;
use actix_web::{web, App, Responder};
use std::io;
use actix_web::middleware::Logger;
use time::Duration;

pub struct ClientController;
impl ClientController {
    #[inline]
    pub async fn run(&self) -> io::Result<()>{
        let config = CFG.clone();
        let secret_key = Key::from(&[0; 64]);
        let redis_store = RedisSessionStore::builder(config.get_redis())
            .build()
            .await
            .unwrap();

        actix_web::HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(Cors::permissive())
                .wrap(
                    SessionMiddleware::builder(
                        redis_store.clone(),
                        secret_key.clone(),
                    )
                        .cookie_name(String::from("SessionID"))
                        .cookie_path(String::from("/"))
                        .cookie_same_site(SameSite::None)
                        .cookie_http_only(false)
                        .session_lifecycle(
                            PersistentSession::default()
                                .session_ttl(Duration::days(30))
                                .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest)
                        )
                        .build()
                )
                .service(
                    web::scope("/v1")
                        .configure(v1_router)
                )
                .route("/",web::get().to(hello))
                .route("/api",web::get().to(hello))
        })
            .bind((config.http.host, config.http.port))?
            .workers(config.http.workers)
            .run().await?;
        Ok(())
    }
}


async fn hello() -> impl Responder{
    "Hello GitData.AI"
}
