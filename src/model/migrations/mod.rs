use lazy_static::lazy_static;
use sea_orm::{ConnectionTrait, DatabaseBackend, DbConn, QueryResult, Statement};
use std::io;
use std::io::ErrorKind;

lazy_static!(
    pub static ref SQL:String = {
        std::fs::read_to_string("init.sql").unwrap()
    };
);

pub struct SqlMigration{
    db: DbConn
}

impl SqlMigration {
    pub fn new(db: DbConn) -> Self {
        Self{
            db,
        }
    }
    pub async fn migrate(&self) -> io::Result<Vec<QueryResult>>{
        let state = Statement::from_string(DatabaseBackend::Postgres, &*SQL);
        match self.db.query_all(state).await{
            Ok(data) => Ok(data),
            Err(e) => Err(io::Error::new(ErrorKind::Other,e))
        }

    }
}