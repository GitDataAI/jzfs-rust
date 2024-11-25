use actix_web::{web, Responder};
use jzfs_module::entity::common::CommonOne;
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::org::team::TeamEntity;
use jzfs_module::entity::uuid::Uuid;
use jzfs_module::Module;

pub async fn team_get_by_name(module: web::Data<Module>, name: web::Json<CommonOne<String>>) -> impl Responder{
    let data = name.data.clone();
    match module.org_team_get_by_name(data).await{
        Ok(data)=>{
            R::<Vec<TeamEntity>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(data),
            }
        }
        Err(e)=>{
            R::<Vec<TeamEntity>>{
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}

pub async fn team_get_by_uid(module: web::Data<Module>, uid: web::Json<CommonOne<Uuid>>) -> impl Responder{
    let data = uid.data.clone();
    match module.org_team_get_by_id(data).await{
        Ok(data)=>{
            R::<TeamEntity>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(data),
            }
        }
        Err(e)=>{
            R::<TeamEntity>{
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}