#![allow(non_upper_case_globals)]
#![allow(unused)]

use lombok::{Builder, Data};
use sea_orm::entity::prelude::*;
use sea_orm::{Condition, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub use Model as AkSk;


#[derive(Serialize,Deserialize,Debug,Clone,Eq, PartialEq,DeriveEntityModel,Data,Builder)]
#[sea_orm(table_name = "aksk")]
pub struct Model{
    #[sea_orm(primary_key)]
    id: Uuid,
    user_id: Uuid,
    access_key: String,
    secret_key: String,
    description: Option<String>,
    created_at: time::OffsetDateTime,
    updated_at: time::OffsetDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize,Deserialize,Debug,Clone,Builder,Data)]
pub struct AkSkParams{
    user_id: Option<Uuid>,
    id: Option<Uuid>,
    access_key: Option<String>,
}
impl Default for AkSkParams {
    fn default() -> Self {
        Self{
            user_id: Default::default(),
            id: Default::default(),
            access_key: Some("".to_string()),
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone,Builder,Data)]
pub struct ListAkSkParams{
    user_id: Option<Uuid>,
    after: Option<time::OffsetDateTime>,
    amount: u64,
}

impl Default for ListAkSkParams {
    fn default() -> Self {
        Self{
            user_id: None,
            after: None,
            amount: 0,
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone,Builder,Data)]
pub struct DeleteAkSkParams{
    user_id:  Option<Uuid>,
    id: Option<Uuid>,
    access_key: Option<String>,
}
impl Default for DeleteAkSkParams {
    fn default() -> Self {
        Self{
            user_id: None,
            id: None,
            access_key: Some("".to_string()),
        }
    }
}

#[derive(Clone)]
pub struct AkSkRepo{
    db: DatabaseConnection
}
impl AkSkRepo{
    pub async fn insert(&self, ak_sk: AkSk) -> anyhow::Result<AkSk>{
        match Entity::insert(ActiveModel::from(ak_sk.clone()))
            .exec(&self.db)
            .await{
            Ok(_) => {
                Ok(ak_sk)
            }
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
    pub async fn get(&self, params: AkSkParams) -> anyhow::Result<AkSk>{
        let mut condition = Condition::all();
        if let Some(id) = params.id{
            condition = condition.add(Expr::col(Column::Id).eq(id))
        }
        if let Some(user_id) = params.user_id{
            condition = condition.add(Expr::col(Column::UserId).eq(user_id))
        }
        if let Some(access_key) = params.access_key{
            condition = condition.add(Expr::col(Column::AccessKey).eq(access_key))
        }
        match Entity::find()
            .filter(condition)
            .one(&self.db)
            .await{
            Ok(ak_sk) => {
                match ak_sk{
                    Some(ak_sk) => {
                        Ok(ak_sk)
                    }
                    None => {
                        Err(anyhow::anyhow!("ak_sk not found"))
                    }
                }
            }
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }
    }
    pub async fn list(&self, params: ListAkSkParams) -> anyhow::Result<Vec<AkSk>>{
        let mut condition = Condition::any();
        if let Some(user_id) = params.user_id{
            condition = condition.add(Expr::col(Column::UserId).eq(user_id))
        }
        if let Some(after) = params.after{
            condition = condition.add(Expr::col(Column::CreatedAt).lt(after))
        }
        match Entity::find()
            .filter(condition)
            .order_by_desc(Column::CreatedAt)
            .limit(params.amount)
            .all(&self.db)
            .await{
            Ok(ak_sk) => {
                Ok(ak_sk)
            }
            Err(e) => {
                Err(anyhow::anyhow!(e))
            }
        }

    }
    pub async fn delete(&self, params: DeleteAkSkParams) ->anyhow::Result<u64>{
        let mut condition = Condition::all();
        if let Some(id) = params.id{
            condition = condition.add(Expr::col(Column::Id).eq(id))
        }
        if let Some(user_id) = params.user_id{
            condition = condition.add(Expr::col(Column::UserId).eq(user_id))
        }
        if let Some(access_key) = params.access_key{
            condition = condition.add(Expr::col(Column::AccessKey).eq(access_key))
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


#[cfg(test)]
mod aksk_tests{
    use sea_orm::DatabaseConnection;
    use tokio::sync::OnceCell;
    use uuid::Uuid;

    const DATABASE_URL: &str = "postgresql://postgres:******@*****/postgres";
    static DB:OnceCell<DatabaseConnection> = OnceCell::const_new();
    async fn init_test(){
        DB.get_or_init(||async {
            let db = sea_orm::Database::connect(DATABASE_URL).await.unwrap();
            db
        }).await;
    }
    #[tokio::test]
    async fn test_insert(){
        init_test().await;
        let db = DB.get().unwrap();
        let repo = super::AkSkRepo{db: db.clone()};
        let ak_sk = super::AkSk{
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            access_key: "".to_string(),
            secret_key: "".to_string(),
            description: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        };
        let result = repo.insert(ak_sk).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get(){
        init_test().await;
        let db = DB.get().unwrap();
        let repo = super::AkSkRepo{db: db.clone()};
        let id = Uuid::new_v4();
        let ak_sk = super::AkSk{
            id: id.clone(),
            user_id: id,
            access_key: "".to_string(),
            secret_key: "".to_string(),
            description: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        };
        let result = repo.insert(ak_sk).await;
        let ak_sk_id = result.unwrap().id;
        let mut params = super::AkSkParams::default();
        params.set_id(Some(ak_sk_id));
        let resp = repo.get(params).await;
        assert!(resp.is_ok());
    }
    #[tokio::test]
    async fn test_list(){
        init_test().await;
        let db = DB.get().unwrap();
        let repo = super::AkSkRepo{db: db.clone()};
        let id = Uuid::new_v4();
        let ak_sk = super::AkSk{
            id: id.clone(),
            user_id: id,
            access_key: "".to_string(),
            secret_key: "".to_string(),
            description: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        };
        let result = repo.insert(ak_sk).await;
        assert!(result.is_ok());
        let mut params = super::ListAkSkParams::default();
        params.set_user_id(Some(id));
        let resp = repo.list(params).await;
        assert!(resp.is_ok());
    }
    #[tokio::test]
    async fn test_delete(){
        init_test().await;
        let db = DB.get().unwrap();
        let repo = super::AkSkRepo{db: db.clone()};
        let id = Uuid::new_v4();
        let ak_sk = super::AkSk{
            id: id.clone(),
            user_id: id,
            access_key: "".to_string(),
            secret_key: "".to_string(),
            description: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        };
        let result = repo.insert(ak_sk).await;
        assert!(result.is_ok());
        let mut params = super::DeleteAkSkParams::default();
        params.set_id(Some(result.unwrap().id));
        let resp = repo.delete(params).await;
        assert!(resp.is_ok());
    }
}