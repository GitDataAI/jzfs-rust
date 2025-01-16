use std::path::PathBuf;

use dotenv::dotenv;
use env_home::env_home_dir;
use log::{error, info};
pub mod api;
pub mod database;
pub mod git;
pub mod rpc;
pub mod email;

pub struct GitDataConfig {}

impl GitDataConfig {
    pub fn init() {
        dotenv().ok();
        info!(".env file loaded");
        let home = match dotenv::var("USER_HOME") {
            Ok(h) => PathBuf::from(h),
            Err(_) => match env_home_dir() {
                Some(path) => path,
                None => {
                    error!("No home found. HOME/USERPROFILE not set or empty");
                    std::process::exit(1);
                }
            },
        };
        info!("Home: {:?}", home);
        if std::fs::read_dir(home.join(".gitdata")).is_err() {
            std::fs::create_dir(home.join(".gitdata"))
                .expect("Failed to create .gitdata directory");
            std::fs::create_dir(home.join(".gitdata/repository"))
                .expect("Failed to create .gitdata/repos directory");
            std::fs::create_dir(home.join(".gitdata/logs"))
                .expect("Failed to create .gitdata/logs directory");
            std::fs::create_dir(home.join(".gitdata/nfs"))
                .expect("Failed to create .gitdata/nfs directory");
            std::fs::create_dir(home.join(".gitdata/static"))
                .expect("Failed to create .gitdata/static");
        } else {
            info!("GitData directory found");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        env_logger::init();
        GitDataConfig::init();
    }
}
