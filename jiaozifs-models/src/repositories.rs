use sqlx::FromRow;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use crate::{ExactMatch, LikeryMatch, MatchMode, PrefixMatch, SuffixMatch, DB};


#[derive(Clone, Debug, PartialEq, Eq,FromRow)]
pub struct Repositiories {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub head: String,
    pub visible: bool,
    pub use_public_storage: bool,
    pub storage_namespace: Option<String>,
    pub storage_adapter_params: Option<String>,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Default for Repositiories {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Repositiories {
            id: Uuid::new_v4(),
            name: "".to_string(),
            owner_id: Uuid::new_v4(),
            head: "".to_string(),
            visible: false,
            use_public_storage: false,
            storage_namespace: None,
            storage_adapter_params: None,
            description: None,
            creator_id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
       }
    }
}

impl Repositiories {
    pub fn id(&mut self,id: Uuid) -> &mut Self{
        self.id = id;
        self
    }
    pub fn name(&mut self,name: String) -> &mut Self{
        self.name = name;
        self
    }
    pub fn owner_id(&mut self,owner_id: Uuid) -> &mut Self{
        self.owner_id = owner_id;
        self
    }
    pub fn head(&mut self,head: String) -> &mut Self{
        self.head = head;
        self
    }
    pub fn visible(&mut self,visible: bool) -> &mut Self{
        self.visible = visible;
        self
    }
    pub fn use_public_storage(&mut self,use_public_storage: bool) -> &mut Self{
        self.use_public_storage = use_public_storage;
        self
    }
    pub fn storage_namespace(&mut self,storage_namespace: Option<String>) -> &mut Self{
        self.storage_namespace = storage_namespace;
        self
    }
    pub fn storage_adapter_params(&mut self,storage_adapter_params: Option<String>) -> &mut Self{
        self.storage_adapter_params = storage_adapter_params;
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

impl Repositiories {
    pub async fn insert(&self) -> anyhow::Result<()> {
        let db = DB.get().unwrap();
        sqlx::query(
            r#"
            INSERT INTO repositories (id, name, owner_id, head, visible, use_public_storage, storage_namespace, storage_adapter_params, description, creator_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.owner_id)
            .bind(&self.head)
            .bind(&self.visible)
            .bind(&self.use_public_storage)
            .bind(&self.storage_namespace)
            .bind(&self.storage_adapter_params)
            .bind(&self.description)
            .bind(&self.creator_id)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(db)
            .await?;
        Ok(())
    }
    pub async fn get(id: Option<Uuid>,creator_id: Option<Uuid>,name: Option<String>,own_id: Option<Uuid>) -> anyhow::Result<Option<Repositiories>>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(creator_id) = creator_id {
            wheres.push(format!("creator_id = '{}'", creator_id));
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name));
        }
        if let Some(own_id) = own_id {
            wheres.push(format!("owner_id = '{}'", own_id));
        }
        let query = format!("SELECT * FROM repositories WHERE {}", wheres.join(" AND "));
        let result = sqlx::query_as::<_, Repositiories>(&query)
            .fetch_optional(db)
            .await?;
        Ok(result)
    }
    pub async fn list(id: Option<Uuid>,creator_id: Option<Uuid>,name: Option<String>,own_id: Option<Uuid>, name_match: Option<MatchMode>,visible: Option<bool>,amount: i64) -> anyhow::Result<Vec<Repositiories>>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(creator_id) = creator_id {
            wheres.push(format!("creator_id = '{}'", creator_id));
        }
        if let Some(visiable) = visible {
            wheres.push(format!("visible = '{}'", visiable));
        }
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

        if let Some(own_id) = own_id {
            wheres.push(format!("owner_id = '{}'", own_id));
        }
        let query = if wheres.is_empty(){
            format!("SELECT * FROM repositories LIMIT {}", amount)
        } else {
            format!("SELECT * FROM repositories WHERE {} LIMIT {}", wheres.join(" AND "), amount)
        };
        let result = sqlx::query_as::<_, Repositiories>(&query)
            .fetch_all(db)
            .await?;
        Ok(result)
    }
    pub async fn delete(id: Option<Uuid>,owner_id: Option<Uuid>, name: Option<String>) -> anyhow::Result<u64> {
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(id) = id {
            wheres.push(format!("id = '{}'", id));
        }
        if let Some(owner_id) = owner_id {
            wheres.push(format!("owner_id = '{}'", owner_id));
        }
        if let Some(name) = name {
            wheres.push(format!("name = '{}'", name));
        }
        if wheres.is_empty(){
            return Err(anyhow::anyhow!("No conditions were specified for deletion."));
        }
        let query = format!("DELETE FROM repositories WHERE {}", wheres.join(" AND "));
        Ok(sqlx::query(&query)
            .execute(db)
            .await?.rows_affected())
    }
    pub async fn update_by_id(id: Uuid, description: Option<String>,visible: Option<bool>,head: Option<String>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let mut wheres = Vec::new();
        if let Some(description) = description {
            wheres.push(format!("description = '{}'", description));
        }
        if let Some(visible) = visible {
            wheres.push(format!("visible = '{}'", visible));
        }
        if let Some(head) = head {
            wheres.push(format!("head = '{}'", head));
        }
        if wheres.is_empty(){
            return Err(anyhow::anyhow!("No conditions were specified for update."));
        }
        let query = format!("UPDATE repositories SET {} WHERE id = '{}'", wheres.join(","), id);
        Ok(sqlx::query(&query)
            .execute(db)
            .await?.rows_affected())
    }
}

#[cfg(test)]
mod repositories_tests {
    use std::env::var;
    use super::*;
    fn init_model() -> Repositiories {
        Repositiories::default()
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
    async fn test_create() {
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get() {
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
        let result = Repositiories::get(Some(model.id),None,None,None).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
    #[tokio::test]
    async fn test_list() {
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
        let result = Repositiories::list(None,None,None,None,None,None,10).await;
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }
    #[tokio::test]
    async fn test_delete() {
        init_db().await.unwrap();
        let model = init_model();
        let result = model.insert().await;
        assert!(result.is_ok());
        let result = Repositiories::delete(Some(model.id),None,None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),1);
    }
}