use actix_session::Session;
use actix_web::{web, Responder};
use base64::Engine;
use deadpool_redis::redis::AsyncCommands;
use config::redis::REDIS;
use config::result::R;
use dto::auth::base64::Base64Inner;
use dto::session::{SessionUserValue, SESSION_USER_KEY};
use dto::users::{UserResetPasswd, UserResetPassword};
use crate::service::Service;

pub async fn api_user_reset_passwd_profile(
    session: Session, 
    dto: web::Json<Base64Inner>,
    service: web::Data<Service>
) -> impl Responder
{
    let model = session.get::<SessionUserValue>(SESSION_USER_KEY).unwrap();
    if model.is_none(){
        return R::<String>{
            code: 400,
            msg: Option::from(
                "[Error] User Not Login".to_string()
            ),
            data: None,
        }
    }
    let model = model.unwrap();
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
    let dto: UserResetPasswd = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    match service.user_service.reset_password(dto,model.uid).await{
        Ok(_) => {
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

pub async fn api_user_reset_passwd_forget(
    dto: web::Json<Base64Inner>,
    service: web::Data<Service>,
)
-> impl Responder
{
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
    let dto: UserResetPassword = match serde_json::from_slice(&dto){
        Ok(dto) => dto,
        Err(e) => {
            return R::<String>{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    };
    let token = dto.token.clone();
    let mut redis = REDIS.get().unwrap().write().unwrap();
    let email = match redis.get::<String,String>(token).await{
        Ok(email) => email,
        Err(_) => {
            return R::<String>{
                code: 400,
                msg: Option::from("[Error] Token Expired".to_string()),
                data: None,
            }
        }
    };
    let uid = match service.user_service.point_user_by_email(email).await{
        Ok(uid) => uid,
        Err(_) => {
            return R::<String>{
                code: 400,
                msg: Option::from("[Error] User Not Exist".to_string()),
                data: None,
            }
        }
    };
    match service.user_service.forget_reset(dto,uid).await{
        Ok(_) => {
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
