use actix_session::Session;
use actix_web::{web, Responder};
use rbatis::rbdc::Uuid;
use config::result::R;
use dto::session::{SessionUserValue, SESSION_USER_KEY};
use model::groups::groups::GroupModel;
use crate::service::Service;

pub async fn api_group_list_owner(
    session: Session,
    service: web::Data<Service>
)
    -> impl Responder
{
    let model = session.get::<SessionUserValue>(SESSION_USER_KEY).unwrap().unwrap();
    match service.group_service.list_by_uid(Uuid(model.uid.to_string())).await{
        Ok(data) => {
            R::<Vec<GroupModel>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Option::from(data),
            }
        }
        Err(e) => {
            R::<Vec<GroupModel>>{
                code: 500,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}