use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::DB;

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct Wips {
    pub id: Uuid,
    pub current_tree: Vec<u8>,
    pub base_commit: Vec<u8>,
    pub repository_id: Uuid,
    pub ref_id: Uuid,
    pub state: String,
    pub creator_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Default for Wips {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Self{
            id: Uuid::new_v4(),
            current_tree: vec![],
            base_commit: vec![],
            repository_id: Uuid::new_v4(),
            ref_id: Uuid::new_v4(),
            state: "open".to_string(),
            creator_id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}
impl Wips {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn current_tree(&mut self,current_tree: Vec<u8>) -> &mut Self{
        self.current_tree = current_tree;
        self
    }
    pub fn base_commit(&mut self,base_commit: Vec<u8>) -> &mut Self{
        self.base_commit = base_commit;
        self
    }
    pub fn repository_id(&mut self,repository_id: Uuid) -> &mut Self{
        self.repository_id = repository_id;
        self
    }
    pub fn ref_id(&mut self,ref_id: Uuid) -> &mut Self{
        self.ref_id = ref_id;
        self
    }
    pub fn state(&mut self,state: String) -> &mut Self{
        self.state = state;
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

impl Wips {
    pub async fn insert(&self) -> anyhow::Result<()> {
        let pool = DB.get().unwrap();
        sqlx::query(r#"INSERT INTO wips (id,current_tree,base_commit,repository_id,ref_id,state,creator_id,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)"#)
            .bind(&self.id)
            .bind(&self.current_tree)
            .bind(&self.base_commit)
            .bind(&self.repository_id)
            .bind(&self.ref_id)
            .bind(&self.state)
            .bind(&self.creator_id)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub async fn get(id:Option<Uuid>,creator_id: Option<Uuid>, repository_id: Option<Uuid>, ref_id: Option<Uuid>) -> anyhow::Result<Wips>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(creator_id) = creator_id {
            wheres.push(format!("creator_id = '{}'", creator_id));
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id));
        }
        if let Some(ref_id) = ref_id {
            wheres.push(format!("ref_id = '{}'", ref_id));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"));
        }
        let query = format!(
            "SELECT * FROM wips WHERE {}",
            wheres.join(" AND ")
        );
        let wips = sqlx::query_as::<_, Wips>(&query)
            .fetch_one(db)
            .await?;
        Ok(wips)
    }
    pub async fn list(creator_id: Option<Uuid>, repository_id: Option<Uuid>, ref_id: Option<Uuid>) -> anyhow::Result<Vec<Wips>>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(creator_id) = creator_id {
            wheres.push(format!("creator_id = '{}'", creator_id));
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id));
        }
        if let Some(ref_id) = ref_id {
            wheres.push(format!("ref_id = '{}'", ref_id));
        }
        let query =  if wheres.is_empty() {
            "SELECT * FROM wips".to_string()
        }else {
            format!(
                "SELECT * FROM wips WHERE {}",
                wheres.join(" AND ")
            )
        };
        let wips = sqlx::query_as::<_, Wips>(&query)
            .fetch_all(db)
            .await?;
        Ok(wips)
    }
    pub async fn delete(creator_id: Option<Uuid>, repository_id: Option<Uuid>, ref_id: Option<Uuid>, id: Option<Uuid>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(creator_id) = creator_id {
            wheres.push(format!("creator_id = '{}'", creator_id));
        }
        if let Some(repository_id) = repository_id {
            wheres.push(format!("repository_id = '{}'", repository_id));
        }
        if let Some(ref_id) = ref_id {
            wheres.push(format!("ref_id = {}", ref_id))
        }
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if wheres.is_empty() {
            return Err(anyhow::anyhow!("No conditions provided"));
        }
        let query = format!(
            "DELETE FROM wips WHERE {}",
            wheres.join(" AND ")
        );
        let result = sqlx::query(&query)
            .execute(db)
            .await?;
        Ok(result.rows_affected())
    }
    pub async fn update_by_id(id: Uuid, updated_at: PrimitiveDateTime, state: Option<String>, current_tree: Option<Vec<u8>> , base_commit: Option<Vec<u8>>) -> anyhow::Result<()>{
        let db = DB.get().unwrap();
        let mut sets = Vec::new();
        if let Some(state) = state {
            sets.push(format!("state = '{}'", state));
        }
        if let Some(current_tree) = current_tree {
            sets.push(format!("current_tree = '{:?}'", current_tree));
        }
        if let Some(base_commit) = base_commit {
            sets.push(format!("base_commit = '{:?}'", base_commit));
        }
        sets.push(format!("updated_at = '{}'", updated_at));
        if !sets.is_empty() {
            let query = format!(
                "UPDATE wips SET {} WHERE id = '{}'",
                sets.join(", "),
                id
            );
            sqlx::query(&query)
                .execute(db)
                .await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No fields to update"))
        }
    }
}


#[cfg(test)]
mod wips_test{
    use std::env::var;
    use super::*;
    fn init_model() -> Wips {
        Wips::default()
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
        let wips = init_model();
        let result = wips.insert().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get(){
        init_db().await.unwrap();
        let wips = init_model();
        let result = wips.insert().await;
        assert!(result.is_ok());
        let wips = Wips::get(Some(wips.id), None, None, None).await;
        assert!(wips.is_ok());
    }
    #[tokio::test]
    async fn test_list(){
        init_db().await.unwrap();
        let wips = init_model();
        let result = wips.insert().await;
        assert!(result.is_ok());
        let wips = Wips::list(None, None, None).await;
        assert!(wips.is_ok());
    }
    #[tokio::test]
    async fn test_delete(){
        init_db().await.unwrap();
        let wips = init_model();
        let result = wips.insert().await;
        assert!(result.is_ok());
        let wips = Wips::delete(None, None, None, Some(wips.id)).await;
        assert!(wips.is_ok());
    }
    #[tokio::test]
    async fn test_update_by_id(){
        init_db().await.unwrap();
        let wips = init_model();
        let result = wips.insert().await;
        assert!(result.is_ok());
        let wips = Wips::update_by_id(wips.id, wips.updated_at, Some("open".to_string()), None, None).await;
        assert!(wips.is_ok());
    }
}