use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use apalis::layers::retry::RetryPolicy;
use apalis::prelude::*;
use apalis_sql::Config;
use apalis_sql::postgres::PgListen;
use apalis_sql::postgres::PostgresStorage;
use deadpool_redis::Runtime;
use gitdata::config::database::DatabaseConfig;
use gitdata::config::database::PgConfig;
use gitdata::model::migrate::DatabaseMigrate;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use tonic::transport::Server;
use tracing::debug;
use tracing::info;
use crate::jobs::email::EmailJobs;
use crate::jobs::email::send_email;
use crate::jobs::sync_repo::{sync_repo, SyncRepoMessage};
use crate::service::rpc::GitCoreRpc;

pub mod auth;
pub mod rpc;
pub mod emails;
pub mod repository;
pub mod users;

#[derive(Clone)]
pub struct AppState {
    pub active_read : DatabaseConnection,
    pub active_write : DatabaseConnection,
    pub deprecated : DatabaseConnection,
    pub email_jobs : Arc<Mutex<PostgresStorage<EmailJobs>>>,
    pub redis_pool : deadpool_redis::Pool,
}

impl AppState {
    pub async fn init() -> anyhow::Result<AppState> {
        info!("Loading database configuration");
        let config = DatabaseConfig::load()?;
        info!("Connect ActiveRead Database Pool");
        let active_read = database_conn(config.pg.get("ActiveRead").unwrap().clone()).await?;
        info!("Connect ActiveWrite Database Pool");
        let active_write = database_conn(config.pg.get("ActiveWrite").unwrap().clone()).await?;
        info!("Connect Deprecated Database Pool");
        let deprecated = database_conn(config.pg.get("Deprecated").unwrap().clone()).await?;
        let jobs_margin = database_conn(config.pg.get("Jobs").unwrap().clone()).await?;
        info!("Connect Jobs Database Pool");
        let jobs_pool =
            apalis_sql::postgres::PgPool::connect(&*config.pg.get("Jobs").unwrap().format())
                .await?;
        let mut sync_jobs = PostgresStorage::new_with_config(
            jobs_pool.clone(),
            Config::new("apalis::Repository::Sync"),
        );
        
        let mut pg =
            PostgresStorage::new_with_config(jobs_pool.clone(), Config::new("apalis::Email"));
        let migrator = PostgresStorage::migrations();
        let mut job_txn = jobs_margin.get_postgres_connection_pool().acquire().await?;
        migrator.run(&mut job_txn).await?;
        let mut listener = PgListen::new(jobs_pool).await?;
        listener.subscribe_with(&mut pg);
        listener.subscribe_with(&mut sync_jobs);
        tokio::spawn(async move {
            listener.listen().await.ok();
        });
        let pc = pg.clone();
        let sync_pg:PostgresStorage<SyncRepoMessage> = sync_jobs.clone();
        info!("Connect Redis Pool");
        let redis_cfg =
            deadpool_redis::Config::from_url(config.redis.get("Session").unwrap().format());
        let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1))?;
        
        let state = AppState {
            active_read,
            active_write,
            deprecated,
            email_jobs : Arc::new(Mutex::new(pg)),
            redis_pool,
        };
        let state_replace = state.clone();
        let rpc_state = state_replace.clone();
        tokio::spawn(async move {
            Monitor::new()
                .register({
                    WorkerBuilder::new("email-jobs")
                        .enable_tracing()
                        .retry(RetryPolicy::retries(5))
                        .backend(pc.clone())
                        .build_fn(send_email)
                })
                .register({
                    WorkerBuilder::new("sync-repo")
                        .data(state_replace.clone())
                        .enable_tracing()
                        .retry(RetryPolicy::retries(5))
                        .backend(sync_pg.clone())
                        .build_fn(sync_repo)
                })
                .on_event(|e| debug!("{e}"))
                .run_with_signal(async {
                    tokio::signal::ctrl_c().await?;
                    info!("Shutting down the system");
                    Ok(())
                })
                .await
                .ok();
        });
        
        tokio::spawn(async move {
            let config = gitdata::config::rpc::RpcConfig::load().expect("failed to load rpc config");
            let git_core = GitCoreRpc::new(rpc_state.clone());
            let health = gitdata::health::service::HealthService::default();
            let addr = config.gitcore_node().expect("failed to load gitcore node").url().parse().expect("failed to parse url");
            info!("Starting GitCore RPC Server at {:?}", addr);
            Server::builder()
                .trace_fn(|x| {
                    tracing::log::info!("Url: {:?} Method: {}", x.uri(), x.method());
                    tracing::Span::current()
                })
                .add_service(gitdata::rpc::health::health_server::HealthServer::new(
                    health,
                ))
                .add_service(
                    gitdata::rpc::git_core::rep_repository_server::RepRepositoryServer::new(git_core),
                )
                .serve(addr)
                .await
                .map_err(|x| io::Error::new(io::ErrorKind::Other, x))
                .unwrap();
        });
        Ok(state.clone())
    }
}

async fn database_conn(url : PgConfig) -> anyhow::Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(url.format());
    opt.max_connections(url.max_connections)
        .min_connections(url.min_connections)
        .connect_timeout(Duration::from_secs(url.connect_timeout))
        .idle_timeout(Duration::from_secs(url.idle_timeout))
        .max_lifetime(Duration::from_secs(url.max_conn_lifetime))
        .sqlx_logging(true)
        .sqlx_logging_level(url.level());
    let db = Database::connect(opt).await?;
    if url.dbname.to_lowercase() != "jobs".to_string() {
        DatabaseMigrate::install(&db).await?;
        DatabaseMigrate::up(&db, None).await?;
    }
    Ok(db)
}
