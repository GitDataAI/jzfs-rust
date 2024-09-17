use anyhow::anyhow;
use sqlx::{query, query_as, FromRow};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::DB;

#[derive(Clone, Debug, PartialEq, Eq,FromRow)]
pub struct Members {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repo_id: Uuid,
    pub group_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
impl Default for Members {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Self {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            repo_id: Uuid::new_v4(),
            group_id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Members {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn user_id(&mut self,user_id: Uuid) -> &mut Self{
        self.user_id = user_id;
        self
    }
    pub fn repo_id(&mut self,repo_id: Uuid) -> &mut Self{
        self.repo_id = repo_id;
        self
    }
    pub fn group_id(&mut self,group_id: Uuid) -> &mut Self{
        self.group_id = group_id;
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

impl Members {
    pub async fn insert(&self) -> anyhow::Result<Members> {
        let db = DB.get().unwrap();
        let query = query_as(r#"INSERT INTO members (id,user_id,repo_id,group_id,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *"#)
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.repo_id)
            .bind(&self.group_id)
            .bind(&self.created_at)
            .bind(&self.updated_at);
        query.fetch_one(db).await.map(|row| row)
            .map_err(|e| e.into())
    }
    pub async fn get(user_id: Option<Uuid>, repo_id: Option<Uuid>, id: Option<Uuid>) -> anyhow::Result<Members> {
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(repo_id) = repo_id {
            wheres.push(format!("repo_id = '{}'", repo_id));
        }
        if let Some(user_id) = user_id {
            wheres.push(format!("user_id = '{}'", user_id));
        }
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if repo_id.is_none() && user_id.is_none() && id.is_none(){
            return Err(anyhow!("Invalid arguments"));
        }
        let sql = format!("SELECT * FROM members WHERE {}", wheres.join(" AND "));
        query_as(&sql)
            .fetch_one(db).await.map(|row| row)
            .map_err(|e| e.into())
    }
    pub async fn list(repo_id: Option<Uuid>) -> anyhow::Result<Vec<Members>>{
        let db = DB.get().unwrap();
        let query = match repo_id {
            Some(repo_id) => query_as(r#"SELECT * FROM members WHERE repo_id = $1"#)
                .bind(repo_id),
            None => query_as(r#"SELECT * FROM members"#),
        };
        query.fetch_all(db).await.map(|rows| rows)
            .map_err(|e| e.into())
    }
    pub async fn update(repo_id: Option<Uuid>, user_id: Option<Uuid>, group_id: Option<Uuid>, updated_at: PrimitiveDateTime) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(repo_id) = repo_id {
            wheres.push(format!("repo_id = '{}'", repo_id));
        }
        if let Some(user_id) = user_id {
            wheres.push(format!("user_id = '{}'", user_id));
        }
        if let Some(group_id) = group_id {
            wheres.push(format!("group_id = '{}'", group_id));
        }
        wheres.push(format!("updated_at = '{}'", updated_at));
        let query = format!("UPDATE members SET updated_at = '{}' WHERE {}", updated_at, wheres.join(" AND "));
        sqlx::query(&query)
            .execute(db)
            .await
            .map(|row| row)
            .map_err(|e| e.into())
            .and_then(|row| Ok(row.rows_affected()))
    }
    pub async fn delete(repo_id: Option<Uuid>, user_id: Option<Uuid>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let query = match (repo_id, user_id) {
            (Some(repo_id), Some(user_id)) => query(r#"DELETE FROM members WHERE repo_id = $1 AND user_id = $2"#)
                .bind(repo_id)
                .bind(user_id),
            (Some(repo_id), None) => query(r#"DELETE FROM members WHERE repo_id = $1"#)
                .bind(repo_id),
            (None, Some(user_id)) => query(r#"DELETE FROM members WHERE user_id = $1"#)
                .bind(user_id),
            _=> return Err(anyhow!("Invalid arguments")),
        };
        query.execute(db).await.map(|row| row)
            .map_err(|e| e.into())
            .and_then(|row| Ok(row.rows_affected()))
    }
}


#[cfg(test)]
mod members_test{
    use std::env::var;
    use super::*;
    fn init_model() -> Members {
        Members::default()
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
    async fn test_members_insert(){
        init_db().await.unwrap();
        let model = init_model();
        let result = Members::insert(&model).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_members_get(){
        init_db().await.unwrap();
        let model = init_model();
        let result = Members::insert(&model).await;
        assert!(result.is_ok());
        let result = Members::get(None, None, Some(model.id)).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_members_list(){
        init_db().await.unwrap();
        let model = init_model();
        let result = Members::insert(&model).await;
        assert!(result.is_ok());
        let result = Members::list(None).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_members_delete(){
        init_db().await.unwrap();
        let model = init_model();
        let result = Members::insert(&model).await;
        assert!(result.is_ok());
        let result = Members::delete(None, Some(model.user_id)).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_members_update(){
        init_db().await.unwrap();
        let model = init_model();
        let result = Members::insert(&model).await;
        assert!(result.is_ok());
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        let result = Members::update(Some(model.repo_id), None, None, now).await;
        assert!(result.is_ok());
    }

}