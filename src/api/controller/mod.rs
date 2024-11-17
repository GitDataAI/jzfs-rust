pub mod v1;
pub(crate) mod session;

use crate::api::controller::v1::v1_router;
use crate::db::auth_db;
use actix_session::config::BrowserSession;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::{web, App};
use std::io;
use actix_web::dev::Service;
use time::Duration;
use crate::config::CFG;

pub struct ClientController;
impl ClientController {
    #[inline]
    pub async fn run(&self) -> io::Result<()>{
        let config = CFG.clone();
        let secret_key = Key::from(&[0; 64]);
        let redis_store = RedisSessionStore::new(config.get_redis())
            .await
            .unwrap();
        actix_web::HttpServer::new(move || {
            App::new()
                .wrap(
                    SessionMiddleware::builder(
                        redis_store.clone(),
                        secret_key.clone(),
                    )
                        .cookie_name(String::from("SessionID"))
                        .cookie_path(String::from("/"))
                        .cookie_same_site(SameSite::Lax)
                        .session_lifecycle(
                            BrowserSession::default()
                                .state_ttl(Duration::days(30))
                        )
                        .build()
                )
                .wrap_fn(|mut x,y|{
                    x.headers_mut().insert("Access-Control-Allow-Origin".parse().unwrap(), "*".parse().unwrap());
                    x.headers_mut().insert("Access-Control-Request-Method".parse().unwrap(), "*".parse().unwrap());
                    x.headers_mut().insert("Access-Control-Request-Headers".parse().unwrap(), "*".parse().unwrap());
                    let fut = y.call(x);

                    Box::pin(async move {
                        let res = fut.await.unwrap();
                        Ok(res)
                    })

                })
                .service(
                    web::scope("/v1")
                        .configure(v1_router)
                )
        })
            .bind((config.http.host, config.http.port))?
            .workers(config.http.workers)
            .run().await?;
        Ok(())
    }
}