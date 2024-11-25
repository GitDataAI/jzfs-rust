use actix_web::{web, Responder};
use jzfs_module::entity::common::CommonOne;
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::dto::auth::AuthRespDto;
use jzfs_module::entity::uuid::Uuid;
use jzfs_module::Module;

pub async fn get_user_by_name(module: web::Data<Module>,data: web::Json<CommonOne<String>>) -> impl Responder{
    match module.users_get_by_name(data.data.clone()).await {
        Ok(data)=>{
            R::<Vec<AuthRespDto>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(data),
            }
        }
        Err(e)=>{
            R::<Vec<AuthRespDto>>{
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}

pub async fn get_user_by_uid(module: web::Data<Module>,data: web::Json<CommonOne<Uuid>>) -> impl Responder{
   match module.users_get_by_id(data.data).await{
       Ok(data) => {
           R::<AuthRespDto>{
               code: 200,
               msg: Option::from("[Ok]".to_string()),
               data: Some(data),
           }
       }
       Err(e) => {
           R::<AuthRespDto>{
               code: 403,
               msg: Option::from(e.to_string()),
               data: None,
           }
       }
   }
}