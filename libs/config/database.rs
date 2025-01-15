use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::log;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub pg: HashMap<String, PgConfig>,
    pub mongo: HashMap<String, MongoDBConfig>,
    pub redis: HashMap<String, RedisConfig>,
}

impl DatabaseConfig {
    pub fn write(&self) -> anyhow::Result<()> {
        let config_file =
            std::env::var("GITDATA_CONFIG_FILE").unwrap_or("./database.toml".to_string());
        let config_dir = std::env::var("GITDATA_CONFIG_DIR").unwrap_or("./config".to_string());
        let config_path = std::path::Path::new(&config_dir).join(&config_file);
        let config_dir = config_path.parent().unwrap();
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)?;
        }
        std::fs::write(config_path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
    pub fn load() -> anyhow::Result<DatabaseConfig> {
        let config_file =
            std::env::var("GITDATA_CONFIG_FILE").unwrap_or("./database.toml".to_string());
        let config_dir = std::env::var("GITDATA_CONFIG_DIR").unwrap_or("./config".to_string());
        let config_path = std::path::Path::new(&config_dir).join(&config_file);
        if !config_path.exists() {
            Self::default().write().ok();
            return Ok(DatabaseConfig::default());
        }
        let config = std::fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config)?)
    }
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        let pg = PgConfig::default();
        let mongo = MongoDBConfig::default();
        let redis = RedisConfig::default();
        let mut pg_hash = HashMap::new();
        pg_hash.insert("Default".to_string(), pg);
        let mut mongo_hash = HashMap::new();
        mongo_hash.insert("Default".to_string(), mongo);
        let mut redis_hash = HashMap::new();
        redis_hash.insert("Default".to_string(), redis);
        DatabaseConfig {
            pg: pg_hash,
            mongo: mongo_hash,
            redis: redis_hash,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PgConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub schema: String,
    pub pool_size: u32,
    pub max_conn_lifetime: u64,
    pub max_conn_lifetime_ms: u64,
    pub idle_timeout: u64,
    pub connect_timeout: u64,
    pub max_connections: u32,
    pub min_connections: u32,
    pub log_level: String,
}

impl Default for PgConfig {
    fn default() -> Self {
        PgConfig {
            host: "localhost".to_string(),
            port: "5432".to_string(),
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            dbname: "postgres".to_string(),
            schema: "public".to_string(),
            pool_size: 10,
            max_conn_lifetime: 60,
            max_conn_lifetime_ms: 60000,
            idle_timeout: 60,
            connect_timeout: 10,
            max_connections: 10,
            min_connections: 1,
            log_level: "info".to_string(),
        }
    }
}

impl PgConfig {
    pub fn format(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }
    pub fn level(&self) -> log::LevelFilter {
        match self.log_level.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MongoDBConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
    pub database: String,
    pub auth_db: String,
    pub pool_size: i32,
}

impl Default for MongoDBConfig {
    fn default() -> Self {
        MongoDBConfig {
            host: "localhost".to_string(),
            port: 27017,
            username: "".to_string(),
            password: "".to_string(),
            database: "".to_string(),
            auth_db: "".to_string(),
            pool_size: 10,
        }
    }
}

impl MongoDBConfig {
    pub fn format(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/?directConnection=true&serverSelectionTimeoutMS=2000&maxPoolSize={}&minPoolSize={}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.pool_size,
            self.pool_size / 10
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: i64,
    pub pool_size: usize,
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            password: None,
            db: 0,
            pool_size: 10,
        }
    }
}

impl RedisConfig {
    pub fn format(&self) -> String {
        match &self.password {
            Some(password) => format!(
                "redis://:{}@{}:{}/{}",
                password, self.host, self.port, self.db
            ),
            None => format!("redis://{}:{}/{}", self.host, self.port, self.db),
        }
    }
}
