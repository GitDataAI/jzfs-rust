use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::DB;

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub encrypted_password: String,
    pub current_sign_in_at: Option<PrimitiveDateTime>,
    pub last_sign_in_at: Option<PrimitiveDateTime>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Default for Users {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Users {
            id: Uuid::new_v4(),
            name: "".to_string(),
            email: Uuid::new_v4().to_string(),
            encrypted_password: Uuid::new_v4().to_string(),
            current_sign_in_at: None,
            last_sign_in_at: None,
            current_sign_in_ip: None,
            last_sign_in_ip: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Users {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn name(&mut self,name: String) -> &mut Self{
        self.name = name;
        self
    }
    pub fn email(&mut self,email: String) -> &mut Self{
        self.email = email;
        self
    }
    pub fn encrypted_password(&mut self,encrypted_password: String) -> &mut Self{
        self.encrypted_password = encrypted_password;
        self
    }
    pub fn current_sign_in_at(&mut self,current_sign_in_at: Option<PrimitiveDateTime>) -> &mut Self{
        self.current_sign_in_at = current_sign_in_at;
        self
    }
    pub fn last_sign_in_at(&mut self,last_sign_in_at: Option<PrimitiveDateTime>) -> &mut Self{
        self.last_sign_in_at = last_sign_in_at;
        self
    }
    pub fn current_sign_in_ip(&mut self,current_sign_in_ip: Option<String>) -> &mut Self{
        self.current_sign_in_ip = current_sign_in_ip;
        self
    }
    pub fn last_sign_in_ip(&mut self,last_sign_in_ip: Option<String>) -> &mut Self{
        self.last_sign_in_ip = last_sign_in_ip;
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

impl Users {
    pub async fn insert(&self) -> anyhow::Result<()> {
        let db = DB.get().unwrap();
        sqlx::query(r#"INSERT INTO users (id, name, email, encrypted_password, current_sign_in_at, last_sign_in_at, current_sign_in_ip, last_sign_in_ip, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7,$8, $9, $10)"#)
        .bind(&self.id)
        .bind(&self.name)
        .bind(&self.email)
        .bind(&self.encrypted_password)
        .bind(&self.current_sign_in_at)
        .bind(&self.last_sign_in_at)
        .bind(&self.current_sign_in_ip)
        .bind(&self.last_sign_in_ip)
        .bind(&self.created_at)
        .bind(&self.updated_at)
        .execute(db)
        .await?;
        Ok(())
    }
    pub async fn get(id: Option<Uuid>, name: Option<String>,email: Option<String>) -> anyhow::Result<Users>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name));
        }
        if let Some(email) = email {
            wheres.push(format!("email = '{}'", email));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions specified"));
        }
        let sql = format!("SELECT * FROM users WHERE {}", wheres.join(" AND "));
        let user = sqlx::query_as::<_, Users>(&sql)
            .fetch_one(db)
            .await?;
        Ok(user)
    }
    pub async fn count(name: Option<String>,email: Option<String>) -> anyhow::Result<i64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name));
        }
        if let Some(email) = email {
            wheres.push(format!("email = '{}'", email));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions specified"));
        }
        let sql = format!("SELECT COUNT(*) FROM users WHERE {}", wheres.join(" AND "));
        let count = sqlx::query_scalar::<_, i64>(&sql)
            .fetch_one(db)
            .await?;
        Ok(count)
    }
    pub async fn get_ep_by_name(name: String) -> anyhow::Result<Users>{
        let db = DB.get().unwrap();
        let sql = format!("SELECT * FROM users WHERE name = '{}'", name);
        let user = sqlx::query_as::<_, Users>(&sql)
            .fetch_one(db)
            .await?;
        Ok(user)
    }
}

#[cfg(test)]
mod users_test{
    use std::env::var;
    use super::*;
    fn init_model() -> Users {
        Users::default()
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
        let user = init_model();
        let result = user.insert().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get(){
        init_db().await.unwrap();
        let mut user = init_model();
        let name = Uuid::new_v4().to_string();
        user.name(name);
        user.insert().await.unwrap();
        let user = Users::get(Some(user.id), None, None).await.unwrap();
        assert_eq!(user.id, user.id);
    }
    #[tokio::test]
    async fn test_count(){
        init_db().await.unwrap();
        let mut user = init_model();
        let name = Uuid::new_v4().to_string();
        user.name(name);
        user.insert().await.unwrap();
        let count = Users::count(Some(user.name), None).await.unwrap();
        assert_eq!(count, 1);
    }
    #[tokio::test]
    async fn test_get_ep_by_name(){
        init_db().await.unwrap();
        let mut user = init_model();
        let name = Uuid::new_v4().to_string();
        user.name(name);
        user.insert().await.unwrap();
        let user = Users::get_ep_by_name(user.name).await.unwrap();
        assert_eq!(user.id, user.id);
    }
    #[tokio::test]
    async fn test_get_ep_by_name_not_found(){
        init_db().await.unwrap();
        let user = Users::get_ep_by_name("not_found".to_string()).await;
        assert!(user.is_err());
    }
}