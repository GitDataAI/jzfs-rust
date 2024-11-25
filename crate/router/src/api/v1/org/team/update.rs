use actix_session::Session;
use actix_web::{web, Responder};
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::USER;
use jzfs_module::entity::dto::auth::AuthRespDto;
use jzfs_module::entity::dto::org::team::TeamUpdateReq;
use jzfs_module::Module;

pub async fn team_update(session: Session,module: web::Data<Module>, dto: web::Json<TeamUpdateReq>) -> impl Responder{
    let model = session.get::<AuthRespDto>(USER).unwrap().unwrap();
    match module.org_team_update(dto.into_inner(), model.uid).await{
        Ok(_) => {
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        }
        Err(e) => {
            R::<String>{
                code: 403,
                msg: Some(format!("[Error] {}", e)),
                data: None,
            }
        }
    }
}