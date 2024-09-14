use lombok::{Builder, Data};
use sea_orm::Condition;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jiaozifs_utlis::{Hash, HashType, Hasher};
pub use Model as Commit;
#[derive(Deserialize, Serialize, Debug, Clone, Data, Builder, Eq)]
pub struct Signature{
    name: String,
    email: String,
    when: time::OffsetDateTime
}


#[derive(Serialize,Deserialize,Debug,Clone,Eq,Clone, PartialEq,DeriveEntityModel,Data,Builder)]
#[sea_orm(table_name = "commit")]
pub struct Model{
    #[sea_orm(primary_key)]
    hash: Hash,
    repository_id: Uuid,
    author: Signature,
    committer: Signature,
    merge_tag: String,
    message: String,
    tree_hash: Hash,
    parent_hashs: Vec<Hash>,
    created_at: time::OffsetDateTime,
    updated_at: time::OffsetDateTime
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


impl Commit {
    pub fn get_hash(&self) -> anyhow::Result<Hash>{
        let mut hasher = Hasher::new(&*[HashType::Md5]);
        // TODO CommitObject
        hasher.write_string(&self.author.name)?;
        hasher.write_string(&self.author.email)?;
        hasher.write_int64(self.author.when.unix_timestamp())?;
        hasher.write_string(&self.committer.name)?;
        hasher.write_string(&self.committer.email)?;
        hasher.write_int64(self.committer.when.unix_timestamp())?;
        hasher.write_string(&self.merge_tag)?;
        hasher.write_string(&self.message)?;
        hasher.write_string(&self.tree_hash.to_hex())?;
        for parent_hash in &self.parent_hashs{
            hasher.write_string(&parent_hash.to_hex())?;
        }
        let resp = hasher.md5.unwrap();
        let slice = resp.decompose().1.get_data();
        Ok(Hash(Vec::from(slice)))
    }
}

#[derive(Deserialize,Serialize,Debug,Clone,Data,Builder)]
pub struct DeleteParams{
    hash: Option<Hash>
}

#[derive(Clone,Data,Builder)]
pub struct CommitRepo{
    db: DatabaseConnection,
    repository_id: Uuid
}

impl CommitRepo{
    pub fn repository_id(&self) -> Uuid{
        self.repository_id
    }
    pub async fn commit(&self, hash: Hash) -> anyhow::Result<Option<Commit>>{
        let condition = Condition::all()
            .add(Expr::col(Column::RepositoryId).eq(self.repository_id))
            .add(Expr::col(Column::Hash).eq(hash));
        let resp = Entity::find()
            .filter(condition)
            .one(&self.db)
            .await?;
        Ok(resp)
    }
    pub async fn insert(&self, commit: Commit) -> anyhow::Result<Commit>{
        match Entity::insert(ActiveModel::from(commit.clone()))
            .exec(&self.db)
            .await{
            Ok(_) => {
                Ok(commit)
            }
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
    pub async fn delete(&self, params: DeleteParams) -> anyhow::Result<u64>{
        let mut condition = Condition::all()
            .add(Expr::col(Column::RepositoryId).eq(self.repository_id));
        if let Some(hash) = params.hash{
            condition = condition.add(Expr::col(Column::Hash).eq(hash))
        }
        match Entity::delete_many()
            .filter(condition)
            .exec(&self.db)
            .await{
            Ok(arco) => Ok(arco.rows_affected),
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
}