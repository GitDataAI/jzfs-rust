use actix_session::Session;
use actix_web::{web, Responder};
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::USER;
use jzfs_module::entity::dto::auth::AuthRespDto;
use jzfs_module::entity::dto::org::team::CreateTeamReq;
use jzfs_module::entity::rbatis::rbdc::db::ExecResult;
use jzfs_module::Module;

pub async fn team_create(session: Session, module: web::Data<Module>, dto: web::Json<CreateTeamReq>) -> impl Responder{
    let model = session.get::<AuthRespDto>(USER).unwrap().unwrap();
    match module.org_team_create(dto.into_inner(), model.uid).await {
        Ok(data) => {
            R::<ExecResult>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Option::from(data),
            }
        }
        Err(e) => {
            R::<ExecResult>{
                code: 500,
                msg: Option::from(format!("[Error] {}", e)),
                data: None,
            }
        }
    }
}