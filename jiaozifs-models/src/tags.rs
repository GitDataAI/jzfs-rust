use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::{ExactMatch, LikeryMatch, MatchMode, PrefixMatch, SuffixMatch, DB};

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct Tags {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub tagger: Option<Uuid>,
    pub target: Option<Vec<u8>>,
    pub message: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
impl Default for Tags {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Self {
            id: Uuid::new_v4(),
            repository_id: Uuid::new_v4(),
            name: "".to_string(),
            tagger: None,
            target: None,
            message: None,
            created_at: now,
            updated_at: now,
        }
    }
}
impl Tags {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn repository_id(&mut self,repository_id: Uuid) -> &mut Self{
        self.repository_id = repository_id;
        self
    }
    pub fn name(&mut self,name: String) -> &mut Self{
        self.name = name;
        self
    }
    pub fn tagger(&mut self,tagger: Option<Uuid>) -> &mut Self{
        self.tagger = tagger;
        self
    }
    pub fn target(&mut self,target: Option<Vec<u8>>) -> &mut Self{
        self.target = target;
        self
    }
    pub fn message(&mut self,message: Option<String>) -> &mut Self{
        self.message = message;
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

impl Tags {
    pub async fn insert(&self) -> anyhow::Result<()> {
        let db = DB.get().unwrap();
        sqlx::query(r#"INSERT INTO tags (id, repository_id, name, tagger, target, message, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
            .bind(&self.id)
            .bind(&self.repository_id)
            .bind(&self.name)
            .bind(&self.tagger)
            .bind(&self.target)
            .bind(&self.message)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(db)
            .await?;
        Ok(())
    }
    pub async fn get(id: Option<Uuid>, repository_id: Option<Uuid>, name: Option<String>) -> anyhow::Result<Self> {
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id));
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"));
        }
        let sql = format!(
            "SELECT * FROM tags WHERE {}",
            wheres.join(" AND ")
        );
        let result = sqlx::query_as::<_, Self>(&sql)
            .fetch_one(db)
            .await?;
        Ok(result)
    }
    pub async fn list(repository_id: Uuid, name: Option<String>, name_match: Option<MatchMode>, after: Option<PrimitiveDateTime>) -> anyhow::Result<Vec<Self>> {
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(name) = name {
            if let Some(name_match) = name_match {
                if name_match == ExactMatch {
                    wheres.push(format!("name = '{}'", name));
                } else if name_match == PrefixMatch {
                    wheres.push(format!("name LIKE '{}%'", name));
                } else if name_match == SuffixMatch {
                    wheres.push(format!("name LIKE '%{}'", name));
                } else if name_match == LikeryMatch {
                    wheres.push(format!("name LIKE '%{}%'", name));
                }
            }else {
                wheres.push(format!("name LIKE '%{}%'", name));
            }
        }
        wheres.push(format!("repository_id = '{}'", repository_id));
        if let Some(after) = after {
            wheres.push(format!("created_at > '{}'", after));
        }
        let sql = format!(
            "SELECT * FROM tags WHERE {} ORDER BY created_at DESC",
            wheres.join(" AND ")
        );
        let result = sqlx::query_as::<_, Self>(&sql)
            .fetch_all(db)
            .await?;
        Ok(result)
    }
    pub async fn delete(id: Option<Uuid>, repository_id: Option<Uuid>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"));
        }
        let sql = format!(
            "DELETE FROM tags WHERE {}",
            wheres.join(" AND ")
        );
        let result = sqlx::query(&sql)
            .execute(db)
            .await?;
        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tags_tests {
    use std::env::var;
    use super::*;
    fn init_model() -> Tags {
        Tags::default()
    }
    async fn init_db() -> anyhow::Result<()>{
        DB.get_or_init(||async {
            let url = var("POSTGRES").unwrap_or("postgres://postgres:1313113a@45.66.150.179:5432/postgres".to_string());
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await.unwrap();
            pool
        }).await;
        Ok(())
    }
    #[tokio::test]
    async fn test_insert() {
        init_db().await.unwrap();
        let tag = init_model();
        let result = tag.insert().await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn test_get() {
        init_db().await.unwrap();
        let tag = init_model();
        tag.insert().await.ok();
        let result = Tags::get(Some(tag.id), None, None).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn test_list() {
        init_db().await.unwrap();
        let tag = init_model();
        let result = Tags::list(tag.repository_id, None, None, None).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn test_delete() {
        init_db().await.unwrap();
        let tag = init_model();
        let result = Tags::delete(Some(tag.id), None).await;
        assert!(result.is_ok())
    }
}