use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Execute, FromRow, Row};
use sqlx::types::Json;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use jiaozifs_utlis::{Hash, HashType, Hasher};
use crate::DB;
use crate::filemode::FileMode;

pub use Trees as Blob;
pub use Trees as TreeNode;
pub use Trees as FileTree;

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize)]
pub struct TreeEntity{
    pub name: String,
    pub id_dir: bool,
    pub hash: Vec<u8>
}
#[derive(Debug,Clone, PartialEq, Eq,Serialize,Deserialize)]
pub enum ObjectType{
    InvalidObject = 0,
    CommitObject = 1,
    TreeObject = 2,
    BlobObject = 3,
    TagObject = 4,
}

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize)]
pub struct Property{
    mode: FileMode
}

#[derive(Clone, Debug, PartialEq, Eq,Serialize,Deserialize, FromRow)]
pub struct Trees {
    pub hash: Vec<u8>,
    pub repository_id: Uuid,
    pub check_sum: Option<Vec<u8>>,
    pub r#type: Json<ObjectType>,
    pub size: Option<i64>,
    pub properties: Json<Property>,
    pub sub_object: Option<Json<Vec<TreeEntity>>>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}


impl Default for Trees {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());
        Trees {
            hash: vec![],
            repository_id: Uuid::new_v4(),
            check_sum: None,
            r#type: Json::from(ObjectType::TreeObject),
            size: None,
            properties: Json(Property{
                mode: FileMode::Empty
            }),
            sub_object: None,
            created_at: now,
            updated_at: now,
        }
    }
}
impl Trees {
    pub fn id(&mut self,id: Vec<u8>) -> &mut Self{
        self.hash = id;
        self
    }
    pub fn repository_id(&mut self,repository_id: Uuid) -> &mut Self{
        self.repository_id = repository_id;
        self
    }
    pub fn check_sum(&mut self,check_sum: Option<Vec<u8>>) -> &mut Self{
        self.check_sum = check_sum;
        self
    }
    pub fn r#type(&mut self,r#type: Json<ObjectType>) -> &mut Self{
        self.r#type = r#type;
        self
    }
    pub fn size(&mut self,size: Option<i64>) -> &mut Self{
        self.size = size;
        self
    }
    pub fn properties(&mut self,properties: Json<Property>) -> &mut Self{
        self.properties = properties;
        self
    }
    pub fn sub_object(&mut self,sub_object: Option<Json<Vec<TreeEntity>>>) -> &mut Self{
        self.sub_object = sub_object;
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
    pub fn hash(&mut self) -> &mut Self{
        self.hash = sha256::digest(self.calculate_hash().unwrap().to_vec()).as_bytes().to_vec();
        self
    }
}
impl Trees {
    pub fn calculate_hash(&mut self) -> anyhow::Result<Hash> {
        let mut hasher = Hasher::new(&[HashType::Md5]);
        hasher.write_string(&serde_json::to_string(&self.r#type).unwrap())?;
        hasher.write_string(&self.repository_id.to_string())?;
        if self.check_sum.is_some() {
            hasher.write(&self.check_sum.as_ref().unwrap())?;
        }
        for (idx, item) in self.properties.to_map() {
            hasher.write_string(&idx)?;
            hasher.write_string(&item)?;
        }
        Ok(hasher.to_md5_hash()?)
    }
}
impl TreeEntity {
    pub fn sort_by_name(sub_objects: &mut [TreeEntity]) {
        sub_objects.sort_by(|a, b| a.name.cmp(&b.name))
    }
    pub fn new_root_tree_entity(hash: Hash) -> TreeEntity {
        TreeEntity {
            name: "".to_string(),
            id_dir: true,
            hash: hash.to_vec(),
        }
    }
    pub fn equals(&self, other: &TreeEntity) -> bool {
        self.hash == other.hash
    }
}
impl Property {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("mode".to_string(), self.mode.to_string());
        map
    }
}

impl FileTree {
    pub async fn insert(&self) -> anyhow::Result<()>{
        let db = DB.get().unwrap();
        query(r#"INSERT INTO trees (hash, repository_id, check_sum, type, size, properties, sub_object, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#)
            .bind(&self.hash)
            .bind(&self.repository_id)
            .bind(&self.check_sum)
            .bind(&self.r#type)
            .bind(&self.size)
            .bind(&self.properties)
            .bind(&self.sub_object)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .execute(db)
            .await?;
        Ok(())
    }
    pub async fn get(hash: Hash,repository_id: Uuid) -> anyhow::Result<Trees> {
        let db = DB.get().unwrap();
        let query = query_as(r#"SELECT * FROM trees WHERE hash = $1 AND repository_id = $2"#)
            .bind(&hash)
            .bind(&repository_id);
        let tree: Trees = query.fetch_one(db).await?;
        Ok(tree)
    }
    pub async fn tree_nodes(hash: Hash,repository_id: Uuid) -> anyhow::Result<Vec<TreeNode>> {
        let db = DB.get().unwrap();
        let query = query_as(r#"SELECT * FROM trees WHERE hash = $1 AND repository_id = $2"#)
            .bind(&hash)
            .bind(&repository_id);
        let tree: Vec<TreeNode> = query.fetch_all(db).await?;
        Ok(tree)
    }
    pub async fn count(repository_id: Uuid) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let query = query(r#"SELECT COUNT(*) FROM trees WHERE repository_id = $1"#)
            .bind(&repository_id);
        let count: u64 = query.fetch_one(db).await?.len() as u64;
        Ok(count)
    }
    pub async fn list(repository_id: Uuid) -> anyhow::Result<Vec<FileTree>>{
        let db = DB.get().unwrap();
        let query = query_as(r#"SELECT * FROM trees WHERE repository_id = $1"#)
            .bind(&repository_id);
        let trees: Vec<FileTree> = query.fetch_all(db).await?;
        Ok(trees)
    }
    pub async fn delete(repository_id: Uuid, hash: Option<Hash>) -> anyhow::Result<u64>{
        let db = DB.get().unwrap();
        let query = if hash.is_some() {
            let hash = hash.unwrap().0;
            query(r#"DELETE FROM trees WHERE repository_id = $1 AND hash = $2"#)
                .bind(&repository_id)
                .bind(hash)
        } else {
            query(r#"DELETE FROM trees WHERE repository_id = $1"#)
                .bind(&repository_id)
        };
        let query = query.execute(db).await?;
        Ok(query.rows_affected())
    }
}


impl Blob {

    pub async fn blob(hash: Hash,repository_id: Uuid) -> anyhow::Result<Blob> {
        let db = DB.get().unwrap();
        let query = query_as(r#"SELECT * FROM trees WHERE hash = $1 AND repository_id = $2 AND "type" = $3"#)
            .bind(&hash)
            .bind(&repository_id)
            .bind(Json::from(ObjectType::BlobObject));
        let tree: Trees = query.fetch_one(db).await?;
        Ok(tree)
    }
}

#[cfg(test)]
mod tree_test{
    use std::env::var;
    use super::*;
    fn init_model() -> Trees {
        Trees::default()
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
    async fn test_tree_insert() -> anyhow::Result<()> {
        init_db().await?;
        let tree = init_model();

        let result = tree.insert().await;
        assert!(result.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_tree_get() -> anyhow::Result<()> {
        init_db().await?;
        let mut tree = init_model();
        tree.hash();
        tree.insert().await?;
        let result = Trees::get(Hash::from(tree.hash.clone()), tree.repository_id).await;
        assert!(result.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_tree_list() -> anyhow::Result<()> {
        init_db().await?;
        let mut tree = init_model();
        tree.hash();
        tree.insert().await?;
        let result = Trees::list(tree.repository_id).await;
        assert!(result.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_tree_delete() -> anyhow::Result<()> {
        init_db().await?;
        let mut tree = init_model();
        tree.hash = Vec::from(tree.calculate_hash()?);
        tree.insert().await?;
        let result = Trees::delete(tree.repository_id, Some(Hash::from(tree.hash.clone()))).await;
        assert!(result.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_tree_count() -> anyhow::Result<()> {
        init_db().await?;
        let mut tree = init_model();
        tree.hash = Vec::from(tree.calculate_hash()?);
        tree.insert().await?;
        let result = Trees::count(tree.repository_id).await;
        assert!(result.is_ok());
        Ok(())
    }
    #[tokio::test]
    async fn test_tree_tree_nodes() -> anyhow::Result<()> {
        init_db().await?;
        let mut tree = init_model();
        tree.hash = Vec::from(tree.calculate_hash()?);
        tree.insert().await?;
        let result = Trees::tree_nodes(Hash::from(tree.hash.clone()), tree.repository_id).await;
        assert!(result.is_ok());
        Ok(())
   }
    #[tokio::test]
    async fn test_tree_blob() -> anyhow::Result<()> {
        init_db().await?;
        let mut tree = init_model();
        tree.hash();
        tree.insert().await?;
        let result = Trees::get(Hash(tree.hash.clone()), tree.repository_id).await;
        assert!(result.is_ok());
        Ok(())
    }
}