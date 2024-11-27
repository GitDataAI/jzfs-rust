use actix_session::Session;
use actix_web::{web, Responder};
use rbatis::rbdc::Uuid;
use config::result::R;
use dto::group::GroupDelete;
use dto::session::{SessionUserValue, SESSION_USER_KEY};
use crate::service::Service;

pub async fn api_group_delete(
    session: Session,
    dto: web::Json<GroupDelete>,
    service: web::Data<Service>
)
    -> impl Responder
{
    let model = session.get::<SessionUserValue>(SESSION_USER_KEY).unwrap().unwrap();
    match service.group_service.delete(dto.into_inner(), Uuid(model.uid.to_string())).await{
        Ok(_) => {
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        }
        Err(e) => {
            R::<String>{
                code: 500,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}