use lombok::Builder;
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use jiaozifs_utlis::{Date, Hash};
use crate::DB;

#[derive(Clone, Debug, PartialEq, Eq,FromRow)]
pub struct Branches {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub commit_hash: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
impl Default for Branches {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        Self{
            id: Uuid::new_v4(),
            repository_id: Uuid::new_v4(),
            commit_hash: Vec::from(Hash::default()),
            name: "default".to_string(),
            description: None,
            creator_id: Uuid::new_v4(),
            created_at: PrimitiveDateTime::new(now.date(),now.time()),
            updated_at: PrimitiveDateTime::new(now.date(),now.time()),
        }
    }
}
impl Branches {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn repository_id(&mut self,repository_id: Uuid) -> &mut Self{
        self.repository_id = repository_id;
        self
    }
    pub fn commit_hash(&mut self,commit_hash: Hash) -> &mut Self{
        self.commit_hash = Vec::from(commit_hash);
        self
    }
    pub fn name(&mut self,name: String) -> &mut Self{
        self.name = name;
        self
    }
    pub fn description(&mut self,description: Option<String>) -> &mut Self{
        self.description = description;
        self
    }
    pub fn creator_id(&mut self,creator_id: Uuid) -> &mut Self{
        self.creator_id = creator_id;
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

impl Branches {
    pub async fn insert(&self) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        sqlx::query("INSERT INTO branches (id, repository_id, commit_hash, name, description, creator_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&self.id)
            .bind(&self.repository_id)
            .bind(&self.commit_hash)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.creator_id)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(db)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| e.into())
    }
    pub async fn get(id: Option<Uuid>, repository_id: Option<Uuid>, name: Option<String>) -> anyhow::Result<Branches>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id))
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id))
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name))
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"))
        }
        let query = format!("SELECT * FROM branches WHERE {}", wheres.join(" AND "));
        sqlx::query_as::<_, Branches>(&query)
            .fetch_one(db)
            .await
            .map_err(|e| e.into())
    }
    pub async fn list(repository_id: Option<Uuid>, name: Option<String>, after: Option<Date>, amount: Option<u64>) -> anyhow::Result<Vec<Self>>{
        let db = DB.get().unwrap();
        let after = after.map(|x|{
            serde_json::to_string(&x.inner).unwrap()
        });
        let mut wheres = Vec::new();
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id))
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name))
        }
        if let Some(after) = after {
            wheres.push(format!("created_at < '{}'", after))
        }
        let query = if wheres.is_empty() {
            format!("SELECT * FROM branches LIMIT {}", amount.unwrap_or(10))
        }else {
            format!("SELECT * FROM branches WHERE {} ORDER BY created_at DESC LIMIT {}", wheres.join(" AND "), amount.unwrap_or(10))
        };
        sqlx::query_as::<_, Branches>(&query)
            .fetch_all(db)
            .await
            .map_err(|e| e.into())
    }
    pub async fn delete(id: Option<Uuid>, repository_id: Option<Uuid>, name: Option<String>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id))
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id))
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name))
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"))
        }
        let query = format!("DELETE FROM branches WHERE {}", wheres.join(" AND "));
        sqlx::query(&query)
            .execute(db)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| e.into())
    }

    pub async fn update_by_id(id: Option<Uuid>, commit_hash: Option<Hash>) -> anyhow::Result<()>{
        let db = DB.get().unwrap();
        let query = match (id, commit_hash) {
            (Some(id), Some(commit_hash)) => format!("UPDATE branches SET commit_hash = '{}' WHERE id = '{}'", commit_hash.to_hex(), id),
            (Some(id), None) => format!("UPDATE branches SET commit_hash = NULL WHERE id = '{}'", id),
            _ => return Err(anyhow::anyhow!("Invalid parameters"))
        };
        sqlx::query(&query)
            .execute(db)
            .await
            .map(|_| ())
            .map_err(|e| e.into())
    }
}


#[cfg(test)]
mod branches_test{
    use std::env::var;
    use super::*;
    async fn init_model() -> Branches {
        Branches::default()
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
        let model = init_model().await;
        let result = Branches::insert(&model).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get(){
        init_db().await.unwrap();
        let model = init_model().await;
        let result = Branches::insert(&model).await;
        assert!(result.is_ok());
        let result = Branches::get(None, Some(model.repository_id), None).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_list(){
        init_db().await.unwrap();
        let model = init_model().await;
        let result = Branches::insert(&model).await;
        assert!(result.is_ok());
        let result = Branches::list(None, None, None,None).await;
        assert!(result.is_ok());

    }
    #[tokio::test]
    async fn test_delete(){
        init_db().await.unwrap();
        let model = init_model().await;
        let result = Branches::insert(&model).await;
        assert!(result.is_ok());
        let result = Branches::delete(Some(model.id), None, None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}