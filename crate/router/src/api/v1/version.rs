use actix_web::{web, Responder};
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::dto::version::Version;
use jzfs_module::Module;

pub async fn version(module: web::Data<Module>) -> impl Responder{
    let version = module.version();
    R::<Version>{
        code: 200,
        msg: Option::from("[Ok]".to_string()),
        data: Option::from(version),
    }
}