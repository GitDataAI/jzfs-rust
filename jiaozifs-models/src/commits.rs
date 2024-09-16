use std::io::Write;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar, Executor, FromRow};
use sqlx::types::Json;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use jiaozifs_utlis::{Hash, HashType, Hasher};
use crate::DB;

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize)]
pub struct Signature{
    pub name: String,
    pub email: String,
    pub when: PrimitiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq,FromRow,Serialize,Deserialize)]
pub struct Commits {
    pub hash: Vec<u8>,
    pub repository_id: Uuid,
    pub author: Json<Signature>,
    pub committer: Json<Signature>,
    pub merge_tag: Option<String>,
    pub message: Option<String>,
    pub tree_hash: Vec<u8>,
    pub parent_hashes: Vec<Vec<u8>>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl Default for Commits {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(),now.time());
        Self {
            hash: Hash::default().0,
            repository_id: Uuid::default(),
            author: Json(Signature{
                name: "".to_string(),
                email: "".to_string(),
                when: now,
            }),
            committer: Json(Signature{
                name: "".to_string(),
                email: "".to_string(),
                when: now,
            }),
            merge_tag: None,
            message: None,
            tree_hash: Hash::default().0,
            parent_hashes: vec![],
            created_at: now,
            updated_at: now,
        }
    }
}
impl Commits {
    pub fn hash(&mut self,hash: Hash) -> &mut Self{
        self.hash = hash.0;
        self
    }
    pub fn repository_id(&mut self,repository_id: Uuid) -> &mut Self{
        self.repository_id = repository_id;
        self
    }
    pub fn author(&mut self,author: Signature) -> &mut Self{
        self.author = Json(author);
        self
    }
    pub fn committer(&mut self,committer: Signature) -> &mut Self{
        self.committer = Json(committer);
        self
    }
    pub fn merge_tag(&mut self,merge_tag: Option<String>) -> &mut Self{
        self.merge_tag = merge_tag;
        self
    }
    pub fn message(&mut self,message: Option<String>) -> &mut Self{
        self.message = message;
        self
    }
    pub fn tree_hash(&mut self,tree_hash: Hash) -> &mut Self{
        self.tree_hash = tree_hash.0;
        self
    }
    pub fn parent_hashes(&mut self,parent_hashes: Vec<Hash>) -> &mut Self{
        self.parent_hashes = parent_hashes.into_iter().map(|h| h.0).collect();
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
    pub fn get_hash(&self) -> anyhow::Result<Hash>{
        let mut hash = Hasher::new(&[HashType::Md5]);
        hash.write_string(&self.committer.name)?;
        hash.write_string(&self.committer.email)?;
        hash.write_string(&self.committer.when.to_string())?;
        hash.write_string(&self.author.name)?;
        hash.write_string(&self.author.email)?;
        hash.write_string(&self.author.when.to_string())?;
        hash.write_string(&self.message.clone().unwrap_or("".to_string()))?;
        hash.write_string(&self.merge_tag.clone().unwrap_or("".to_string()))?;
        hash.write(&self.tree_hash)?;
        for parent_hash in &self.parent_hashes{
            hash.write(&parent_hash)?;
        }
        Ok(Hash(Vec::from(hash.md5.unwrap().decompose().1.get_data())))
    }
}

impl Commits {
    pub async fn commit(&self,hash: Hash,cr_repository_id: Uuid) -> anyhow::Result<Hash>{
        let db = DB.get().unwrap();
        let query = query_as::<_,Commits>(r#"SELECT * FROM commits WHERE hash = $1 AND repository_id = $2"#)
        .bind(&hash)
        .bind(&cr_repository_id);
        query.fetch_one(db)
            .await
            .map_err(|e| e.into())
            .and_then(|e| Ok(Hash(e.hash)))
    }
    pub async fn insert(&self) -> anyhow::Result<()>{
        let db = DB.get().unwrap();
        let tree = Hash(self.clone().tree_hash).to_hex().as_bytes().to_vec();
        let hash = &self.get_hash()?.0;
        let parent_hashes: Vec<Vec<u8>> = self.parent_hashes.iter().map(|h| Hash(h.clone()).to_hex().as_bytes().to_vec()).collect();
        let query =
            query(r#"INSERT INTO commits (hash, repository_id, author, committer, merge_tag, message, tree_hash, parent_hashes, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#
            )
            .bind(hash)
            .bind(&self.repository_id)
            .bind(&self.author)
            .bind(&self.committer)
            .bind(&self.merge_tag)
            .bind(&self.message)
            .bind(tree)
            .bind(parent_hashes)
            .bind(&self.created_at)
            .bind(&self.updated_at);
        query.execute(db).await?;
        Ok(())
    }
    pub async fn delete(cr_repository_id: Uuid, hash: Option<Hash>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        if hash.is_some(){
            query(r#"DELETE FROM commits WHERE repository_id = $1 AND hash = $2"#)
            .bind(&cr_repository_id)
            .bind(&hash.unwrap().0)
            .execute(db)
            .await
            .map_err(|e| e.into())
            .and_then(|r| Ok(r.rows_affected()))
        }else{
            query(r#"DELETE FROM commits WHERE repository_id = $1"#)
            .bind(&cr_repository_id)
            .execute(db)
            .await
            .map_err(|e| e.into())
            .and_then(|r| Ok(r.rows_affected()))
        }

    }
}

#[cfg(test)]
mod commits_test{
    use std::env::var;
    use super::*;
    async fn init_model() -> Commits {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(),now.time());
        Commits{
            hash: Hash::default().0,
            repository_id: Uuid::default(),
            author: Json(Signature{
                name: "".to_string(),
                email: "".to_string(),
                when: now,
            }),
            committer: Json(Signature{
                name: "".to_string(),
                email: "".to_string(),
                when: now,
            }),
            merge_tag: None,
            message: None,
            tree_hash: vec![],
            parent_hashes: vec![],
            created_at: now,
            updated_at: now,
        }
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
    async fn test_commit(){
        init_db().await.unwrap();
        let mut model = init_model().await;
        let hash = model.get_hash().unwrap();
        model.hash(model.get_hash().unwrap());
        model.insert().await.unwrap();

        let result = model.commit(hash.clone(), model.repository_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), hash);
    }
    #[tokio::test]
    async fn test_insert(){
        init_db().await.unwrap();
        let model = init_model().await;
        let result = model.insert().await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete(){
        init_db().await.unwrap();
        let model = init_model().await;
        let result =   Commits::delete(model.repository_id, Some(model.get_hash().unwrap())).await;
        assert!(result.is_ok());
    }
}