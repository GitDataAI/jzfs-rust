use std::io;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use uuid::Uuid;
use crate::services::AppState;
use crate::services::statistics::repo::{STAR, WATCH};
use crate::model::origin::organization;
use crate::model::repository::repository;
use crate::model::users::{star, users, watch};

impl AppState {
    pub async fn repo_get_by_uid(&self, uid: Uuid) -> io::Result<repository::Model> {
        repository::Entity::find_by_id(uid)
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "repo not found"))
    }
    async fn find_user_or_org(&self, username: &str) -> io::Result<uuid::Uuid> {
        if let Some(user) = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        {
            return Ok(user.uid);
        }

        organization::Entity::find()
            .filter(organization::Column::Username.eq(username))
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .map(|org| org.uid)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "owner not found"))
    }

    pub async fn repo_info(&self, owner: String, repo: String) -> io::Result<repository::Model> {
        let owner_uid = self.find_user_or_org(&owner).await?;

        repository::Entity::find()
            .filter(repository::Column::OwnerId.eq(owner_uid))
            .filter(repository::Column::Name.eq(repo))
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "repo not found"))
    }

    async fn validate_user_and_repo(&self, user_uid: Uuid, repo_uid: Uuid) -> io::Result<(users::Model, repository::Model)> {
        let user = users::Entity::find_by_id(user_uid)
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "user not found"))?;

        let repo = repository::Entity::find_by_id(repo_uid)
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "repo not found"))?;

        Ok((user, repo))
    }
    

    pub async fn repo_star(&self, users_uid: Uuid, repos_uid: Uuid) -> io::Result<()> {
        let (user, repo) = self.validate_user_and_repo(users_uid, repos_uid).await?;

        let existing_star = star::Entity::find()
            .filter(star::Column::UserId.eq(user.uid))
            .filter(star::Column::RepositoryId.eq(repo.uid))
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        let txn = self.write.begin().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        match existing_star {
            Some(record) => {
                star::Entity::delete_by_id(record.uid)
                    .exec(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsStar, Expr::col(repository::Column::NumsStar).sub(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            },
            None => {
                star::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    user_id: Set(user.uid),
                    repository_id: Set(repo.uid),
                    created_at: Set(Utc::now().naive_utc()),
                }.insert(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsStar, Expr::col(repository::Column::NumsStar).add(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }
        }
        txn.commit().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        self.statistics_repo(repos_uid, STAR.to_string()).await.ok();
        Ok(())
    }

    pub async fn repo_watch(&self, users_uid: Uuid, repos_uid: Uuid, level: i32) -> io::Result<()> {
        let (user, repo) = self.validate_user_and_repo(users_uid, repos_uid).await?;

        let existing_watch = watch::Entity::find()
            .filter(watch::Column::UserId.eq(user.uid))
            .filter(watch::Column::RepositoryId.eq(repo.uid))
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let txn = self.write.begin().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        match existing_watch {
            Some(record) => {
                watch::Entity::delete_by_id(record.uid)
                    .exec(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsWatch, Expr::col(repository::Column::NumsWatch).sub(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            },
            None => {
                watch::ActiveModel {
                    uid: Set(Uuid::new_v4()),
                    user_id: Set(user.uid),
                    repository_id: Set(repo.uid),
                    created_at: Set(Utc::now().naive_utc()),
                    level: Set(level),
                }.insert(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                repository::Entity::update_many()
                    .col_expr(repository::Column::NumsWatch, Expr::col(repository::Column::NumsWatch).add(1))
                    .filter(repository::Column::Uid.eq(repo.uid))
                    .exec(&txn)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            }
        }
        txn.commit().await.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        self.statistics_repo(repos_uid, WATCH.to_string()).await.ok();
        Ok(())
    }
}
