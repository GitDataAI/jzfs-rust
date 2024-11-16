use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::OnceCell;
use crate::config::CFG;

pub static REPODB:OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init(){
    REPODB.get_or_init(||async {
        let mut opt = ConnectOptions::new(CFG.get_auth_database());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let db = Database::connect(opt).await.unwrap();
        db
    }).await;
}