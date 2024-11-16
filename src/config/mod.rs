use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Http{
    pub port: u16,
    pub host: String,
    pub workers: usize
}


#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Database{
    pub auth: Auth,
    pub repo: Repo
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Auth{
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub pool: i32
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Repo{
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub pool: i32
}
#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Redis{
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Config{
    pub http: Http,
    pub database: Database,
    pub redis: Redis,
}

impl Default for Config {
    fn default() -> Self {
        Self{
            http: Http {
                port: 34513,
                host: "0.0.0.0".to_string(),
                workers: 8,
            },
            database: Database {
                auth: Auth {
                    hostname: "localhost".to_string(),
                    port: 5432,
                    username: "postgres".to_string(),
                    password: "123456".to_string(),
                    database: "auth".to_string(),
                    pool: 4,
                },
                repo: Repo{
                    hostname: "localhost".to_string(),
                    port: 5432,
                    username: "postgres".to_string(),
                    password: "123456".to_string(),
                    database: "repo".to_string(),
                    pool: 4,
                }
            },
            redis: Redis {
                host: "127.0.0.1".to_string(),
                port: 6379,
            },
        }
    }
}

lazy_static!{
    pub static ref CFG: Config = Config::init();
}

impl Config {
    pub fn get_auth_database(&self) -> String{
        format!("postgres://{}:{}@{}:{}/{}",
        self.database.auth.username,
        self.database.auth.password,
        self.database.auth.hostname,
        self.database.auth.port,
        self.database.auth.database)
    }
    pub fn get_repo_database(&self) -> String{
        format!("postgres://{}:{}@{}:{}/{}",
                self.database.repo.username,
                self.database.repo.password,
                self.database.repo.hostname,
                self.database.repo.port,
                self.database.repo.database)
    }
    pub fn get_http(&self) -> String{
        format!("{}:{}", self.http.host, self.http.port)
    }
    pub fn get_redis(&self) -> String{
        format!("redis://{}:{}",self.redis.host, self.redis.port)
    }
    pub fn init() -> Self {
        if std::fs::read_dir("./config").is_err() {
            std::fs::write("./config", toml::to_string(&Config::default()).unwrap()).unwrap();
            return Config::default();
        }
        if let Ok(buf) = std::fs::read("./config/default.toml") {
            if let Ok(buf) = std::str::from_utf8(&buf) {
                if let Ok(config) = toml::from_str::<Config>(buf) {
                    config
                } else {
                    std::fs::write("./config/default.toml", toml::to_string(&Config::default()).unwrap()).unwrap();
                    Config::default()
                }
            } else {
                std::fs::write("./config/default.toml", toml::to_string(&Config::default()).unwrap()).unwrap();
                Config::default()
            }
        } else {
            Config::default()
        }
    }
}