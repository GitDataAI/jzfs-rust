pub use jzfs_entity as entity;
pub use jzfs_service as service;
use entity::rbatis::RBatis;
use jzfs_entity::db;
use jzfs_service::deadpool_redis::Pool;
use jzfs_service::email::EmailService;
use jzfs_service::redis::Redis;

pub mod users;
pub mod auth;
pub mod email;
pub mod setting;
pub mod repo;
mod org;

#[allow(unused)]
#[derive(Clone)]
pub struct Module{
    db: RBatis,
    email: EmailService,
    redis: Pool
}

impl Module {
    pub async fn init() -> Module {
        let redis = Redis::init();
        let db = db::init().await;
        let email = EmailService::init();
        Self{
            db,
            email,
            redis: redis.pool.clone()
        }
    }
}