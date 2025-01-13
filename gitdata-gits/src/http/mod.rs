mod pack;
mod refs;
mod text;
use std::net::SocketAddr;

use actix_web::web::scope;
use actix_web::{App, HttpServer, web};
use tracing::info;

pub async fn http() -> std::io::Result<()> {
    info!("Starting HTTP server in 0.0.0.0:31245");
    let addr: SocketAddr = ([0, 0, 0, 0], 31245).into();
    let server = HttpServer::new(|| {
        App::new().service(
            scope("/{owner}/{repository}")
                .route("/info/refs", web::to(refs::info_refs))
                .route("/git-upload-pack", web::to(pack::pack))
                .route("/git-receive-pack", web::to(pack::pack))
                .route("/HEAD", web::to(text::text))
                .route("/object/info/{any}", web::to(text::text))
                .route("/object/pack/{any}", web::to(text::text)),
        )
    })
    .bind(addr)?
    .max_connections(usize::MAX)
    .run();
    if !cfg!(test) {
        server.await?;
    }
    Ok(())
}

#[cfg(test)]
mod http_tests {
    use super::*;
    #[tokio::test]
    async fn test_http_starter() {
        assert!(http().await.is_ok());
    }
}
