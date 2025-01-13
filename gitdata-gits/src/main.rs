use std::io;

use log::info;
use tonic::transport::Server;

pub mod health;
pub mod http;
pub mod mount;
pub mod rpc;
pub mod service;
pub mod ssh;
#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().init();
    let http = tokio::spawn(http::http());
    let health = tokio::spawn(async {
        let health = gitdata::health::service::HealthService::default();
        info!("starting health service");
        Server::builder()
            // .trace_fn(|x|{
            //     info!("Url: {:?} Method: {}", x.uri(),x.method());
            //     tracing::Span::current()
            // })
            .add_service(gitdata::rpc::health::health_server::HealthServer::new(
                health,
            ))
            .serve("0.0.0.0:50051".parse().unwrap())
            .await
            .map_err(|x| io::Error::new(io::ErrorKind::Other, x))
            .unwrap();
    });
    while let Ok(_) = tokio::signal::ctrl_c().await {
        http.abort();
        health.abort();
        info!("shutting down");
        break;
    }

    Ok(())
}
