use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use config::result::R;
use dto::auth::base64::Base64Inner;
use dto::session::{ALLOW_NEXT_KEY, SESSION_USER_KEY};
use dto::users::UserApply;
use crate::service::Service;

pub async fn api_user_apply(
    session: Session, 
    dto: web::Json<Base64Inner>,
    service: web::Data<Service>
) -> impl Responder
{
    let allow = session.get::<bool>(ALLOW_NEXT_KEY).unwrap();
    if allow.is_none(){
        return R::<String>{
            code: 400,
            msg: Option::from("[Error] Not Allow Next".to_string()),
            data: None,
        }
    }
    let dto = match base64::prelude::BASE64_STANDARD.decode(dto.inner.as_bytes()){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    let dto: UserApply = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.user_service.apply(dto).await{
        Ok(info) => {
            session.insert(SESSION_USER_KEY, info).unwrap();
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        },
        Err(e) => {
            R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}