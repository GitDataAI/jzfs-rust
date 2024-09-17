use serde::{Deserialize, Serialize};
use sqlx::postgres::PgTypeInfo;
use sqlx::{Database,  Postgres, Type};
use time::OffsetDateTime;

#[derive(Serialize,Deserialize, Clone,Eq, PartialEq,Debug)]
pub struct Date{
    pub inner: OffsetDateTime
}

impl Date {
    pub fn unix_timestamp(&self) -> i64 {
        self.inner.unix_timestamp()
    }
}
impl Default for Date {
    fn default() -> Self {
        Self{
            inner: OffsetDateTime::now_utc()
        }
    }
}


impl Type<Postgres> for Date {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("TIMESTAMP")
    }
}