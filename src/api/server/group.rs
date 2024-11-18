use crate::db::auth_db::AUTHDB;
use crate::db::model::{groups, users};
use sea_orm::prelude::Expr;
use sea_orm::*;
use std::io;
use std::io::ErrorKind;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

pub struct GroupServer {
    auth_db: DatabaseConnection
}

impl Default for GroupServer {
    fn default() -> Self {
        Self{
            auth_db: AUTHDB.get().unwrap().clone()
        }
    }
}

impl GroupServer {
    pub fn new() -> Self{
        Self::default()
    }
    pub async fn create_group(&self, name: String, bio: String, owner_id: Uuid, contact: String) -> io::Result<groups::Model>{
        match (groups::ActiveModel{
            uid: Set(Uuid::new_v4()),
            name: Set(name),
            avatar_url: Set(None),
            bio: Set(bio),
            location: Set(None),
            links: Set(Vec::new()),
            users: Set(vec![owner_id]),
            topics: Set(Vec::new()),
            pinned: Set(Vec::new()),
            header: Set(owner_id),
            contact: Set(contact),
            create_to: Default::default(),
            create_at: Default::default(),
            update_at: Default::default(),
        }
            .insert(&self.auth_db)
            .await){
            Ok(data) => {
                Ok(data)
            }
            Err(e) => {
                Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))
            }
        }
    }
    pub async fn get_from_uid(&self, uid: Uuid) -> io::Result<groups::Model>{
        match groups::Entity::find_by_id(uid)
            .one(&self.auth_db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other,e))?{
            None => {
                Err(io::Error::new(ErrorKind::NotFound,"Group Not Found"))
            }
            Some(data) => {
                Ok(data)
            }
        }
    }
    pub async fn rename(&self,uid: Uuid, group_name: String, new_name: String, header: Uuid) -> io::Result<UpdateResult>{
        groups::Entity::update_many()
            .filter(groups::Column::Uid.eq(uid))
            .filter(groups::Column::Header.eq(header))
            .filter(groups::Column::Name.eq(group_name))
            .col_expr(groups::Column::Name, Expr::value(Value::String(Option::from(Box::new(new_name)))))
            .exec(&self.auth_db)
            .await
            .map_err(|e|io::Error::new(ErrorKind::Other, e))
    }
    pub async fn rebio(&self,uid: Uuid, bio: String,header: Uuid) -> io::Result<UpdateResult>{
        groups::Entity::update_many()
            .filter(groups::Column::Uid.eq(uid))
            .filter(groups::Column::Header.eq(header))
            .col_expr(groups::Column::Bio, Expr::value(Value::String(Option::from(Box::new(bio)))))
            .exec(&self.auth_db)
            .await
            .map_err(|e|io::Error::new(ErrorKind::Other, e))
    }
    pub async fn relocation(&self, uid: Uuid, location: String, header: Uuid) -> io::Result<UpdateResult>{
        groups::Entity::update_many()
            .filter(groups::Column::Uid.eq(uid))
            .filter(groups::Column::Header.eq(header))
            .col_expr(groups::Column::Location, Expr::value(Value::String(Option::from(Box::new(location)))))
            .exec(&self.auth_db)
            .await
            .map_err(|e|io::Error::new(ErrorKind::Other, e))
    }
    pub async fn add_link(&self,uid: Uuid, link: String, header: Uuid) -> io::Result<()>{
        let model = groups::Entity::find()
            .filter(groups::Column::Uid.eq(uid))
            .filter(groups::Column::Header.eq(header))
            .one(&self.auth_db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        if model.is_none() {
            return  Err(io::Error::new(ErrorKind::NotFound,"Group Not Found"))
        }
        let mut model = model.unwrap().into_active_model();
        let mut links = model.links.unwrap();
        links.push(link);
        model.links = Set(links);
        match model
            .update(&self.auth_db)
            .await {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                Err(io::Error::new(ErrorKind::Other,e))
            }
        }
    }
    pub async fn add_user(&self, uid: Uuid,user_id: Uuid, header: Uuid) -> io::Result<()>{
        let gmodel = groups::Entity::find()
            .filter(groups::Column::Uid.eq(uid))
            .filter(groups::Column::Header.eq(header))
            .one(&self.auth_db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        if gmodel.is_none() {
            return  Err(io::Error::new(ErrorKind::NotFound,"Group Not Found"))
        }
        let umodel = users::Entity::find()
            .filter(users::Column::Uid.eq(user_id))
            .one(&self.auth_db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        if umodel.is_none() {
            return  Err(io::Error::new(ErrorKind::NotFound,"User Not Found"))
        }
        let mut uarch = umodel.unwrap().into_active_model();
        let mut garch = gmodel.unwrap().into_active_model();
        let mut gusers = garch.users.unwrap();
        let mut ugroups = uarch.groups.unwrap();
        gusers.push(user_id);
        ugroups.push(uid);
        uarch.groups = Set(ugroups);
        garch.users = Set(gusers);
        self.auth_db.transaction::<_,(),DbErr>(|txn|{
            Box::pin(async move{
                uarch.update(txn).await?;
                garch.update(txn).await?;
                Ok(())
            })
        }).await.map_err(|e|io::Error::new(ErrorKind::Other,e))?;
        Ok(())
    }
    pub async fn remove_user(&self, uid: Uuid, header: Uuid,user_id: Uuid) -> io::Result<()>{
        let gmodel = groups::Entity::find()
            .filter(groups::Column::Uid.eq(uid))
            .filter(groups::Column::Header.eq(header))
            .one(&self.auth_db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;
        if gmodel.is_none() {
            return  Err(io::Error::new(ErrorKind::NotFound,"Group Not Found"))
        }
        let umodel = users::Entity::find()
            .filter(users::Column::Uid.eq(user_id))
            .one(&self.auth_db)
            .await
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        if umodel.is_none() {
            return  Err(io::Error::new(ErrorKind::NotFound,"User Not Found"))
        }
        let mut uarch = umodel.unwrap().into_active_model();
        let mut garch = gmodel.unwrap().into_active_model();
        let mut gusers = garch.users.unwrap();
        let mut ugroups = uarch.groups.unwrap();
        gusers.retain(|x| *x != user_id);
        ugroups.retain(|x| *x != uid);
        uarch.groups = Set(ugroups);
        garch.users = Set(gusers);
        self.auth_db.transaction::<_,(),DbErr>(|txn|{
            Box::pin(async move{
                uarch.update(txn).await?;
                garch.update(txn).await?;
                Ok(())
            })
        }).await.map_err(|e|io::Error::new(ErrorKind::Other,e))?;
        Ok(())
    }
}