use std::io;
use std::io::ErrorKind;
use sea_orm::*;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use crate::db::model::stars;
use crate::db::repo_db::REPODB;

pub struct StarsServer{
    db: DatabaseConnection
}

impl Default for StarsServer {
    fn default() -> Self {
        Self{
            db: REPODB.get().unwrap().clone()
        }
    }
}

impl StarsServer {
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn star(&self, owner_id: Uuid, repo_id: Uuid) -> io::Result<stars::Model>{
        let mut model = stars::Entity::find()
            .filter(stars::Column::OwnerId.eq(owner_id))
            .one(&self.db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        let mut vis = false;
        if model.is_none(){
            vis = true;
            model = Some(stars::Model{
                uid: Default::default(),
                owner_id,
                stars_repo: vec![],
            })

        }
        let mut model = model.unwrap().into_active_model();
        let mut repos = model.stars_repo.unwrap();
        repos.push(repo_id);
        model.stars_repo = Set(repos);
        if vis{
            model.insert(&self.db).await.map_err(|e| io::Error::new(ErrorKind::Other, e))
        }else {
            model.update(&self.db).await.map_err(|e| io::Error::new(ErrorKind::Other, e))
        }
    }
    pub async fn unstar(&self, owner_id: Uuid, repo_id: Uuid) -> io::Result<stars::Model>{
        let mut model = stars::Entity::find()
            .filter(stars::Column::OwnerId.eq(owner_id))
            .one(&self.db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        let mut vis = false;
        if model.is_none(){
            vis = true;
            model = Some(stars::Model{
                uid: Default::default(),
                owner_id,
                stars_repo: vec![],
            })
        }
        let mut model = model.unwrap().into_active_model();
        if vis{
            model.insert(&self.db).await.map_err(|e| io::Error::new(ErrorKind::Other, e))
        }else {
            let mut repos = model.stars_repo.unwrap();
            repos.push(repo_id);
            model.stars_repo = Set(repos);
            model.update(&self.db).await.map_err(|e| io::Error::new(ErrorKind::Other, e))
        }
    }
    pub async fn owner(&self, owner_id: Uuid) -> io::Result<stars::Model>{
        let mut model = stars::Entity::find()
            .filter(stars::Column::OwnerId.eq(owner_id))
            .one(&self.db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        let mut vis = false;
        if model.is_none(){
            vis = true;
            model = Some(stars::Model{
                uid: Default::default(),
                owner_id,
                stars_repo: vec![],
            })
        };
        if vis {
            let model = model.unwrap().into_active_model();
            model.insert(&self.db).await.map_err(|e| io::Error::new(ErrorKind::Other, e))
        }else {
            Ok(model.unwrap())
        }
    }
}