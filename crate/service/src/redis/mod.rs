use deadpool_redis::{Config, Pool, Runtime};
use jzfs_config::tracing::info;

pub mod queue;


#[derive(Clone)]
pub struct Redis{
    pub pool: Pool
}

impl Redis {
    pub fn init() -> Redis {
        let redis_url = "redis://192.168.23.128:6379/0";
        info!("redis connect");
        let redis_cfg = Config::from_url(redis_url);
        let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
        Self{
            pool: redis_pool
        }
    }
}