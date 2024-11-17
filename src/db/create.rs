use sea_orm::DatabaseConnection;

use sea_orm::{ConnectionTrait, Schema};
use tracing::{error, info};
use crate::db::{auth_db, repo_db};
use crate::db::auth_db::AUTHDB;
use crate::db::model::{brand, commit, filetrees, groups, public_key, public_token, repo, stars, users};



pub struct InitDatabase{
    auth_db: DatabaseConnection,
    repo_db: DatabaseConnection,
}

impl InitDatabase {
    pub async fn init(){
        auth_db::init().await;
        repo_db::init().await;
        let sf = Self{
            auth_db: AUTHDB.get().unwrap().clone(),
            repo_db: AUTHDB.get().unwrap().clone(),
        };
        info!("Database Connect Successful");
        // info!("Init Database Auth");
        // sf.init_auth().await;
        // info!("Init Database Repo");
        // sf.init_repo().await;
    }
    pub async fn init_auth(&self){
        let builder = self.auth_db.get_database_backend();
        let schema = Schema::new(builder);
        let mut collect = Vec::new();
        collect.push(builder.build(&schema.create_table_from_entity(users::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(groups::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(public_key::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(public_token::Entity)));
        let db = self.auth_db.clone();
        for idx in collect{
            match db.execute(idx).await{
                Ok(exec) => {
                    info!("Create Table( `rows_affected` ): {}",exec.rows_affected())
                }
                Err(err) => {
                    error!("Create Table: {}",err)
                }
            }
        }
    }
    pub async fn init_repo(&self){
        let builder = self.repo_db.get_database_backend();
        let schema = Schema::new(builder);
        let mut collect = Vec::new();
        collect.push(builder.build(&schema.create_table_from_entity(repo::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(commit::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(filetrees::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(brand::Entity)));
        collect.push(builder.build(&schema.create_table_from_entity(stars::Entity)));
        let db = self.repo_db.clone();
        for idx in collect{
            match db.execute(idx).await{
                Ok(exec) => {
                    info!("Create Table( `rows_affected` ): {}",exec.rows_affected())
                }
                Err(err) => {
                    error!("Create Table: {}",err)
                }
            }
        }
    }
}
