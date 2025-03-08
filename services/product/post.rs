use std::io;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::product::data_product;
use crate::model::repository::repository;
use crate::services::AppState;


#[derive(Deserialize,Serialize)]
pub struct DataProductPostParma {
    pub name: String,
    pub description: Option<String>,
    pub license: String,
    pub price: Option<i64>,
    pub hash: String,
    pub r#type: String,
}

impl AppState {
    pub async fn product_data_post(
        &self,
        user_uid: Uuid,
        parma: DataProductPostParma,
        repo_uid: Uuid,
    ) -> io::Result<()> {
        let repo = repository::Entity::find_by_id(repo_uid)
            .one(&self.read)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "repository not found"))?
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "repository not found"))?;
        match self.user_access_owner_model(user_uid).await {
            Ok(x) => {
                if !x.iter().any(|x| x.repos.iter().any(|x|x.uid == repo_uid)){
                    return Err(io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"));
                }
            },
            Err(_) => {
                return Err(io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"));
            }
        }
        let path = format!(
            "{}/{}/{}",
            crate::http::GIT_ROOT,
            repo.node_uid,
            repo.uid
        );
        let blob = crate::blob::GitBlob::new(path.into())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let size = blob.size(parma.hash.clone())?;
        let product_model = data_product::ActiveModel {
            uid: Set(Uuid::new_v4()),
            name: Set(parma.name),
            description: Set(parma.description),
            license: Set(parma.license),
            price: Set(parma.price),
            hash: Set(parma.hash),
            size: Set(size),
            owner: Set(user_uid),
            created_at: Set(chrono::Local::now().naive_local()),
            updated_at: Set(chrono::Local::now().naive_local()),
            repository_uid: Default::default(),
            r#type: Set(parma.r#type),
        };
        product_model.insert(&self.write).await.map_err(|_| io::Error::new(io::ErrorKind::Other, "insert product error"))?;
        Ok(())
    }
}