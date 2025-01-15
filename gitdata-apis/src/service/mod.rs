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
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use tracing::debug;
use tracing::info;

use crate::jobs::email::EmailJobs;
use crate::jobs::email::send_email;

pub mod auth;
pub mod core_git_rpc;
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
        info!("Connect Jobs Database Pool");
        let jobs_pool =
            apalis_sql::postgres::PgPool::connect(&*config.pg.get("Jobs").unwrap().format())
                .await?;
        let mut pg =
            PostgresStorage::new_with_config(jobs_pool.clone(), Config::new("apalis::Email"));
        let mut listener = PgListen::new(jobs_pool).await?;
        listener.subscribe_with(&mut pg);
        tokio::spawn(async move {
            listener.listen().await.ok();
        });
        let pc = pg.clone();
        tokio::spawn(async move {
            Monitor::new()
                .register({
                    WorkerBuilder::new("email-jobs")
                        .enable_tracing()
                        .retry(RetryPolicy::retries(5))
                        .backend(pc.clone())
                        .build_fn(send_email)
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
        info!("Connect Redis Pool");
        let redis_cfg =
            deadpool_redis::Config::from_url(config.redis.get("Session").unwrap().format());
        let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(AppState {
            active_read,
            active_write,
            deprecated,
            email_jobs : Arc::new(Mutex::new(pg)),
            redis_pool,
        })
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
    Ok(db)
}
