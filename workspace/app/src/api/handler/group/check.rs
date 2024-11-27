use actix_session::Session;
use actix_web::{web, Responder};
use config::result::R;
use dto::group::{GroupCheckNoName, GroupOwnerCheck};
use dto::session::{SessionUserValue, SESSION_USER_KEY};
use crate::service::Service;

pub async fn api_group_check_name(
    service: web::Data<Service>,
    dto: web::Json<GroupCheckNoName>
)
    -> impl Responder
{
    match service.group_service.check_no_name(dto.into_inner().name).await{
        Ok(r) => {
            R::<String>{
                code: if r{ 200 }else { 201 },
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

pub async fn api_group_owner_check(
    session: Session,
    service: web::Data<Service>,
    dto: web::Json<GroupOwnerCheck>
)
    -> impl Responder
{
    let model = session.get::<SessionUserValue>(SESSION_USER_KEY).unwrap().unwrap();
    match service.group_service.check_group_owner(dto.into_inner().uid, model.uid).await{
        Ok(r) => {
            R::<String>{
                code: if r{ 200 }else { 201 },
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