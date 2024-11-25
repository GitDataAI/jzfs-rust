use actix_session::Session;
use actix_web::{web, Responder};
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::USER;
use jzfs_module::entity::dto::auth::AuthRespDto;
use jzfs_module::entity::dto::org::team::TeamDeleteReq;
use jzfs_module::entity::rbatis::rbdc::db::ExecResult;
use jzfs_module::Module;

pub async fn team_delete(session: Session, module: web::Data<Module>, dto: web::Json<TeamDeleteReq>) -> impl Responder{
    let model = session.get::<AuthRespDto>(USER).unwrap().unwrap();
    match module.org_team_delete(dto.into_inner(),model.uid).await{
        Ok(ok) => {
            R::<ExecResult>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Option::from(ok),
            }
        }
        Err(e) => {
            R::<ExecResult>{
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}