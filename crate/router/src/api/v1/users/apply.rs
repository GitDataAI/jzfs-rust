use actix_session::Session;
use actix_web::{web, Responder};
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::ALLOWNEXT;
use jzfs_module::entity::dto::users::ApplyDto;
use jzfs_module::entity::rbatis::rbdc::db::ExecResult;
use jzfs_module::Module;

pub async fn apply(session: Session, apply: web::Json<ApplyDto>, module: web::Data<Module>) -> impl Responder{
    let allow = session.get::<bool>(ALLOWNEXT).unwrap();
    if allow.is_none() { 
        return R::<ExecResult>{
            code: 403,
            msg: Option::from("[Service 10001] Permission denied".to_string()),
            data: None,
        }
    }
    let allow = allow.unwrap();
    if !allow {
        return R::<ExecResult>{
            code: 403,
            msg: Option::from("[Service 10001] Permission denied Not Allow Next".to_string()),
            data: None,
        }
    }
    match module.users_apply(apply.username.clone(), apply.email.clone(), apply.passwd.clone()).await {
        Ok(result) => {
            R::<ExecResult> {
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(result),
            }
        }
        Err(e) => {
            R::<ExecResult> {
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}