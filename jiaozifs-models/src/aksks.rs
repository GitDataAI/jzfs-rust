use lombok::{Builder, Data};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use jiaozifs_utlis::Date;
use crate::DB;

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct AkSk {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_key: String,
    pub secret_key: String,
    pub description: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
impl Default for AkSk{
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        AkSk{
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            access_key: Uuid::new_v4().to_string(),
            secret_key: Uuid::new_v4().to_string(),
            description: Some("default".to_string()),
            created_at: PrimitiveDateTime::new(now.date(),now.time()),
            updated_at: PrimitiveDateTime::new(now.date(),now.time()),
        }
    }
}
impl AkSk {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn user_id(&mut self,user_id: Uuid) -> &mut Self{
        self.user_id = user_id;
        self
    }
    pub fn access_key(&mut self,access_key: String) -> &mut Self{
        self.access_key = access_key;
        self
    }
    pub fn secret_key(&mut self,secret_key: String) -> &mut Self{
        self.secret_key = secret_key;
        self
    }
    pub fn description(&mut self,description: Option<String>) -> &mut Self{
        self.description = description;
        self
    }
    pub fn created_at(&mut self,created_at: PrimitiveDateTime) -> &mut Self{
        self.created_at = created_at;
        self
    }
    pub fn updated_at(&mut self,updated_at: PrimitiveDateTime) -> &mut Self{
        self.updated_at = updated_at;
        self
    }
}

impl AkSk {
    pub async fn insert(&self) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let rec = sqlx::query("INSERT INTO aksks (id,user_id,access_key,secret_key,description,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7)")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.access_key)
            .bind(&self.secret_key)
            .bind(&self.description)
            .bind(self.created_at)
            .bind(self.updated_at)
            .execute(db).await?;
        Ok(rec.rows_affected())
    }
    pub async fn get(user_id: Option<Uuid>, id: Option<Uuid>,access_key: Option<String>) -> anyhow::Result<AkSk>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(user_id) = user_id {
            wheres.push(format!("user_id = '{}'", user_id));
        }
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(access_key) = access_key {
            wheres.push(format!("access_key = '{}'", access_key));
        }
        let query = if !wheres.is_empty() {
            format!("SELECT * FROM aksks WHERE {}", wheres.join(" AND "))
        } else {
            return Err(anyhow::anyhow!("Invalid parameters"));
        };
        Ok(sqlx::query_as::<_, AkSk>(&query).fetch_one(db).await?)
    }
    pub async fn list(user_id: Option<Uuid>,after: Option<OffsetDateTime>, amount: Option<u64>) -> anyhow::Result<Vec<AkSk>>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(user_id) = user_id {
            wheres.push(format!("user_id = '{}'", user_id));
        }
        if let Some(after) = after {
            wheres.push(format!("created_at > '{}'", serde_json::to_string(&after)?));
        }
        if let Some(amount) = amount {
            wheres.push(format!("LIMIT {}", amount));
        }
        let query = if !wheres.is_empty() {
            format!("SELECT * FROM aksks WHERE {}", wheres.join(" AND "))
        } else {
            "SELECT * FROM aksks".to_string()
        };
        Ok(sqlx::query_as::<_, AkSk>(&query).fetch_all(db).await?)
    }
    pub async fn delete(user_id: Option<Uuid>, id: Option<Uuid>,access_key: Option<String>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(user_id) = user_id {
            wheres.push(format!("user_id = '{}'", user_id));
        }
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(access_key) = access_key {
            wheres.push(format!("access_key = '{}'", access_key));
        }
        if wheres.is_empty() {
           return Err(anyhow::anyhow!("Invalid parameters"));
        }
        let query = format!("DELETE FROM aksks WHERE {}", wheres.join(" AND "));
        Ok(sqlx::query(&query).execute(db).await?.rows_affected())
    }
}

#[cfg(test)]
mod aksks_test{
    use std::env::var;
    use super::*;
    fn init_model() -> AkSk {
        AkSk::default()
    }
    async fn init_db() -> anyhow::Result<()>{
        DB.get_or_init(||async {
            let url = var("POSTGRES").unwrap();
            let pool = sqlx::postgres::PgPoolOptions::new()

                .max_connections(50)
                .connect(&url)
                .await.unwrap();
            pool
        }).await;
        Ok(())
    }
    #[tokio::test]
    async fn test_insert(){
        init_db().await.unwrap();
        let model = init_model();
        let res = model.insert().await;
        assert!(res.is_ok())
    }
    #[tokio::test]
    async fn test_get(){
        init_db().await.unwrap();
        let model = init_model();
        let _ = model.insert().await;
        let res = AkSk::get(None, Some(model.id), None).await;
        assert!(res.is_ok());
    }
    #[tokio::test]
    async fn test_list(){
        init_db().await.unwrap();
        let model = init_model();
        let res = model.insert().await;
        assert!(res.is_ok());
        let res = AkSk::list(None, None, None).await;
        assert!(res.is_ok());
    }
    #[tokio::test]
    async fn test_delete(){
        init_db().await.unwrap();
        let model = init_model();
        let res = model.insert().await;
        assert!(res.is_ok());
        let res = AkSk::delete(None, Some(model.id), None).await;
        assert!(res.is_ok());
    }

}