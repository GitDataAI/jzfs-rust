#![feature(exit_status_error)]

use std::io;

use gitdata::config;
use log::info;
use tonic::transport::Server;

use crate::mount::git::NodeStorage;

pub mod health;
pub mod http;
pub mod mount;
pub mod rpc;
pub mod service;
pub mod ssh;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    info!("starting gitdata");
    let mut pool = mount::StoragePool::new();
    pool.add_node("default".to_string(), NodeStorage {
        root : std::env::current_dir()?.join("/data"),
    });
    let http = tokio::spawn(http::http(pool.clone()));
    let health = tokio::spawn(async move {
        let health = gitdata::health::service::HealthService::default();
        let core_git = rpc::core_git::CoreGit::new(pool.clone());
        info!("starting health service");
        let config = config::rpc::RpcConfig::load().expect("failed to load rpc config");
        let addr = config
            .coregit_node()
            .expect("failed to load gitcore node")
            .url()
            .parse()
            .expect("failed to parse url");
        info!("connecting to gitcore node at {:?}", addr);
        Server::builder()
            .trace_fn(|x| {
                info!("Url: {:?} Method: {}", x.uri(), x.method());
                tracing::Span::current()
            })
            .add_service(gitdata::rpc::health::health_server::HealthServer::new(
                health,
            ))
            .add_service(
                gitdata::rpc::core_git::rep_repository_server::RepRepositoryServer::new(core_git),
            )
            .serve(addr)
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
