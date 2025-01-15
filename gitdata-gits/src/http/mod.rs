mod pack;
mod refs;
mod text;
use std::net::SocketAddr;
use std::time::Duration;

use actix_web::App;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::web::scope;
use gitdata::config;
use tokio::time::sleep;
use tracing::info;

use crate::mount::StoragePool;
use crate::rpc;

pub async fn http(pool : StoragePool) -> anyhow::Result<()> {
    info!("Starting HTTP server in 0.0.0.0:31245");
    // 延迟初始化，确保rpc服务全部启动
    sleep(Duration::from_secs(10)).await;
    let rpc = config::rpc::RpcConfig::load()?;
    let git_core = rpc::git_core::RepositoryRpc::new(rpc.gitcore_node().unwrap().url()).await?;
    let addr : SocketAddr = ([0, 0, 0, 0], 31245).into();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(git_core.clone()))
            .service(
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
