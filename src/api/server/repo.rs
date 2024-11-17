use std::io;
use std::io::Error;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::{Expr, Json};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::db::model::repo;
use crate::db::model::repo::{ActiveModel, RepoOrigin};
use crate::db::repo_db::REPODB;

pub struct RepoServer{ db: DatabaseConnection }


impl Default for RepoServer {
    fn default() -> Self {
        RepoServer{
            db: REPODB.get().unwrap().clone()
        }
    }
}

impl RepoServer{
    pub fn new() -> RepoServer {
        Self::default()
    }
    pub async fn get_owner_repo(&self,owner_id: RepoOrigin) -> io::Result<Vec<repo::Model>>{
        let origin = serde_json::to_value(&owner_id)?;
        repo::Entity::find()
            .filter(repo::Column::Origin.eq(origin))
            .all(&self.db.clone())
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn create_repo(&self, repo: repo::ActiveModel) -> Result<InsertResult<ActiveModel>, Error> {
        repo::Entity::insert(repo)
            .exec(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn update_visible(&self, repo_id: Uuid, owner_id: RepoOrigin, visible: bool) -> Result<UpdateResult, Error> {
        let origin = serde_json::to_value(&owner_id)?;
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(repo_id))
            .filter(repo::Column::Origin.eq(origin))
            .col_expr(repo::Column::Visible, Expr::value(Value::Bool(Option::from(visible))))
            .col_expr(repo::Column::UpdateAt, Expr::value(Value::TimeDateTimeWithTimeZone(Option::from(Box::new(OffsetDateTime::now_utc())))))
            .exec(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn update_bio(&self, repo_id: Uuid, owner_id: RepoOrigin, bio: String) -> Result<UpdateResult, Error> {
        let origin = serde_json::to_value(&owner_id)?;
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(repo_id))
            .filter(repo::Column::Origin.eq(origin))
            .col_expr(repo::Column::Bio, Expr::value(Value::String(Option::from(Box::new(bio)))))
            .col_expr(repo::Column::UpdateAt, Expr::value(Value::TimeDateTimeWithTimeZone(Option::from(Box::new(OffsetDateTime::now_utc())))))
            .exec(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn move_repo_origin(&self, repo_id: Uuid, old_owner_id: RepoOrigin, new_owner_id: RepoOrigin) -> io::Result<UpdateResult> {
        let old_origin = serde_json::to_value(&old_owner_id)?;
        let new_origin = serde_json::to_value(&new_owner_id)?;
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(repo_id))
            .filter(repo::Column::Origin.eq(old_origin))
            .col_expr(repo::Column::Origin, Expr::value(Value::Json(Option::from(Box::new(new_origin)))))
            .col_expr(repo::Column::UpdateAt, Expr::value(Value::TimeDateTimeWithTimeZone(Option::from(Box::new(OffsetDateTime::now_utc())))))
            .exec(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn forks(&self, repo_id: Uuid, owner_id: RepoOrigin) -> io::Result<repo::Model>{
        let model = repo::Entity::find_by_id(repo_id)
            .one(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))?;
        if model.is_none() {
            return Err(Error::new(io::ErrorKind::NotFound, "Repository not found"));
        }
        let origin = serde_json::to_string(&owner_id)?;
        let mut arch = model.unwrap().into_active_model();
        arch.fork_from = Set(Some(repo_id));
        arch.uid = Set(Uuid::new_v4());
        arch.origin = Set(Json::from(origin));
        arch.update_at = Set(OffsetDateTime::now_utc());
        arch.insert(&self.db).await.map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn add_star(&self, repo_id: Uuid) -> Result<repo::Model, Error> {
        let model = repo::Entity::find()
            .filter(repo::Column::Uid.eq(repo_id))
            .one(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))?;
        if model.is_none(){
            return Err(Error::new(io::ErrorKind::NotFound, "Repository not found"));
        }
        let mut model = model.unwrap().into_active_model();
        model.stars = Set(model.stars.unwrap() + 1);
        model.update(&self.db).await.map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn div_star(&self, repo_id: Uuid) -> Result<repo::Model, Error> {
        let model = repo::Entity::find()
            .filter(repo::Column::Uid.eq(repo_id))
            .one(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))?;
        if model.is_none(){
            return Err(Error::new(io::ErrorKind::NotFound, "Repository not found"));
        }
        let mut model = model.unwrap().into_active_model();
        model.stars = Set(model.stars.unwrap() - 1);
        model.update(&self.db).await.map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
    pub async fn update_time_now(&self, repo_id: Uuid) -> io::Result<UpdateResult>{
        repo::Entity::update_many()
            .filter(repo::Column::Uid.eq(repo_id))
            .col_expr(repo::Column::UpdateAt, Expr::value(Value::TimeDateTimeWithTimeZone(Option::from(Box::new(OffsetDateTime::now_utc())))))
            .exec(&self.db)
            .await
            .map_err(|e| Error::new(io::ErrorKind::Other, e))
    }
}