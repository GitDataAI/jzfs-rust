use actix_session::Session;
use actix_web::{web, Responder};
use config::result::R;
use dto::session::{SessionUserValue, SESSION_USER_KEY};
use dto::users::UserUpdate;
use crate::service::Service;

pub async fn api_user_update(
    session: Session,
    service: web::Data<Service>,
    dto: web::Json<UserUpdate>
)
    -> impl Responder
{
    let user = session.get::<SessionUserValue>(SESSION_USER_KEY).unwrap();
    if user.is_none() {
        return R::<String> {
            code: 401,
            msg: Option::from("[Error] User Not Login".to_string()),
            data: None,
        }
    }
    let user = user.unwrap();
    match service.user_service.update(dto.into_inner(), user.uid).await {
        Ok(info) => {
            session.insert(SESSION_USER_KEY, info).unwrap();
            R::<String> {
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String> {
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}