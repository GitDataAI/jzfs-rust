#![allow(dead_code)]

use sqlx::{query, FromRow};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::DB;

type MergeState = i64;
const MergeStateInit: MergeState = 1;
const MergeStateMerged: MergeState = 2;
const MergeStateClosed: MergeState = 3;


#[derive(Clone, Debug, PartialEq, Eq,FromRow)]
pub struct MergeRequests {
    pub id: Uuid,
    pub mr_sequence: i64,
    pub source_repo_id: Uuid,
    pub target_repo_id: Uuid,
    pub target_branch_id: Uuid,
    pub source_branch_id: Uuid,
    pub title: String,
    pub merge_state: MergeState,
    pub description: Option<String>,
    pub author_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Default for MergeRequests {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        MergeRequests {
            id: Uuid::new_v4(),
            mr_sequence: 0,
            source_repo_id: Uuid::new_v4(),
            target_repo_id: Uuid::new_v4(),
            target_branch_id: Uuid::new_v4(),
            source_branch_id: Uuid::new_v4(),
            title: "".to_string(),
            merge_state: MergeStateInit,
            description: None,
            author_id:Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl MergeRequests {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn mr_sequence(&mut self,mr_sequence: i64) -> &mut Self{
        self.mr_sequence = mr_sequence;
        self
    }
    pub fn source_repo_id(&mut self,source_repo_id: Uuid) -> &mut Self{
        self.source_repo_id = source_repo_id;
        self
    }
    pub fn target_repo_id(&mut self,target_repo_id: Uuid) -> &mut Self{
        self.target_repo_id = target_repo_id;
        self
    }
    pub fn target_branch_id(&mut self,target_branch_id: Uuid) -> &mut Self{
        self.target_branch_id = target_branch_id;
        self
    }
    pub fn source_branch_id(&mut self,source_branch_id: Uuid) -> &mut Self{
        self.source_branch_id = source_branch_id;
        self
    }
    pub fn title(&mut self,title: String) -> &mut Self{
        self.title = title;
        self
    }
    pub fn merge_state(&mut self,merge_state: MergeState) -> &mut Self{
        self.merge_state = merge_state;
        self
    }
    pub fn description(&mut self,description: Option<String>) -> &mut Self{
        self.description = description;
        self
    }
    pub fn author_id(&mut self,author_id: Uuid) -> &mut Self{
        self.author_id = author_id;
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

impl MergeRequests {
    pub async fn insert(&self) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let query = query(r#"INSERT INTO merge_requests (id, mr_sequence, source_repo_id, target_repo_id, target_branch_id, source_branch_id, title, merge_state, description, author_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#)
            .bind(&self.id)
            .bind(&self.mr_sequence)
            .bind(&self.source_repo_id)
            .bind(&self.target_repo_id)
            .bind(&self.target_branch_id)
            .bind(&self.source_branch_id)
            .bind(&self.title)
            .bind(&self.merge_state)
            .bind(&self.description)
            .bind(&self.author_id)
            .bind(&self.created_at)
            .bind(&self.updated_at);
        query.execute(db).await.map(|r| r.rows_affected())
            .map_err(|e| e.into())
    }
    pub async fn get(id: Option<Uuid>, sequence: Option<i64>, target_repo_id: Option<Uuid>, target_branch_id: Option<Uuid>, source_branch_id: Option<Uuid>,merge_state: MergeState) -> anyhow::Result<MergeRequests>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(sequence) = sequence {
            wheres.push(format!("mr_sequence = {:?}", sequence));
        }
        if let Some(target_repo_id) = target_repo_id {
            wheres.push(format!("target_repo_id = '{}'", target_repo_id));
        }
        if let Some(target_branch_id) = target_branch_id {
            wheres.push(format!("target_branch_id = '{}'", target_branch_id));
        }
        if let Some(source_branch_id) = source_branch_id {
            wheres.push(format!("source_branch_id = '{}'", source_branch_id));
        }
        wheres.push(format!("merge_state = {}", merge_state));
        let query = format!(r#"SELECT * FROM merge_requests WHERE {}"#, wheres.join(" AND "));
        dbg!(query.clone());
        sqlx::query_as::<_, MergeRequests>(&query)
            .fetch_one(db)
            .await
            .map_err(|e| e.into())
    }
    pub async fn list(target_repo_id: Option<Uuid>,merge_state: Option<MergeState>,after: Option<OffsetDateTime>, amount: u64) -> anyhow::Result<Vec<MergeRequests>>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(target_repo_id) = target_repo_id {
            wheres.push(format!("target_repo_id = '{}'", target_repo_id));
        }
        if let Some(merge_state) = merge_state {
            wheres.push(format!("merge_state = {}", merge_state));
        }
        if let Some(after) = after {
            wheres.push(format!("updated_at > '{}'", after));
        }
        let query = if wheres.is_empty() {
            format!(r#"SELECT * FROM merge_requests ORDER BY updated_at DESC LIMIT {}"#, amount)
        }else {
            format!(r#"SELECT * FROM merge_requests WHERE {} ORDER BY updated_at DESC LIMIT {}"#, wheres.join(" AND "), amount)
        };
        sqlx::query_as::<_, MergeRequests>(&query)
            .fetch_all(db)
            .await
            .map_err(|e| e.into())
    }
    pub async fn delete(mr_sequence: Option<i64>, target_repo_id: Option<Uuid>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(mr_sequence) = mr_sequence {
            wheres.push(format!("mr_sequence = {}", mr_sequence));
        }
        if let Some(target_repo_id) = target_repo_id {
            wheres.push(format!("target_repo_id = '{}'", target_repo_id));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"));
        }
        let query = format!(r#"DELETE FROM merge_requests WHERE {}"#, wheres.join(" AND "));
        sqlx::query(&query)
            .execute(db)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| e.into())
    }
    pub async fn update_by_id(mr_sequence: i64, target_repo_id: Uuid, updated_at: PrimitiveDateTime, title: Option<String>, description: Option<String>, merge_state: Option<MergeState>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        wheres.push(format!("mr_sequence = {}", mr_sequence));
        wheres.push(format!("target_repo_id = '{}'", target_repo_id));
        if let Some(title) = title {
            wheres.push(format!("title = '{}'", title));
        }
        if let Some(description) = description {
            wheres.push(format!("description = '{}'", description));
        }
        if let Some(merge_state) = merge_state {
            wheres.push(format!("merge_state = {}", merge_state));
        }
        let query = format!(r#"UPDATE merge_requests SET updated_at = '{}' WHERE {}"#, updated_at, wheres.join(" AND "));
        sqlx::query(&query)
            .execute(db)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| e.into())
    }
}


#[cfg(test)]
mod merge_requests_test{
    use std::env::var;
    use super::*;
    fn init_model() -> MergeRequests {
        MergeRequests::default()
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
    async fn test_merge_requests_insert(){
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn test_merge_requests_get(){
        init_db().await.unwrap();
        let mut model = init_model();
        model.merge_state(MergeStateMerged);
        let result = model.insert().await;
        assert!(result.is_ok());
        let result = MergeRequests::get(Some(model.id), None, None, None, None, MergeStateMerged).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_merge_requests_list(){
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
        let result = MergeRequests::list(None, None, None, 0).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn test_merge_requests_delete(){
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
        let result = MergeRequests::delete(Some(model.mr_sequence), Some(model.target_repo_id)).await;
        assert!(result.is_ok())
    }
    #[tokio::test]
    async fn test_merge_requests_update(){
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        let result = MergeRequests::update_by_id(1, Uuid::new_v4(), now, Some("test".to_string()), Some("test".to_string()), Some(1)).await;
        assert!(result.is_ok())
    }
}