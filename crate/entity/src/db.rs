use rbatis::{DefaultPool, RBatis};
use rbatis::rbdc::pool::conn_manager::ConnManager;
use rbatis::rbdc::pool::Pool;
use rbdc_pg::PgDriver;
use tokio::sync::OnceCell;

pub static DB:OnceCell<RBatis> = OnceCell::const_new();

pub async fn init() -> RBatis {
    DB.get_or_init(||async {
        let db = RBatis::new();
        let conf = ConnManager::new(
            PgDriver{},
            "postgres://postgres:123456@192.168.23.128/jzfs"
        ).unwrap();
        let pool = DefaultPool::new(conf).unwrap();
        pool.set_max_open_conns(20).await;
        db.init_pool(pool).unwrap();
        db
    }).await.clone()
}