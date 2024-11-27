use actix_session::Session;
use actix_web::{web, Responder};
use config::result::R;
use dto::session::{ALLOW_NEXT_KEY, CAPTCHA};
use dto::users::{UserCaptchaEmail, UserCaptchaEmailCheck};
use crate::service::Service;

pub async fn api_email_rand_captcha(
    session: Session, 
    service: web::Data<Service>,
    dto: web::Json<UserCaptchaEmail>
)
    -> impl Responder
{
        match service.email_service.generate_and_send_captcha(dto.email.clone()).await{
            Ok(result) => {
                session.insert(CAPTCHA, result).ok();
                R::<String>{
                    code: 200,
                    msg: Option::from("[Ok]".to_string()),
                    data: None,
                }
            }
            Err(e) => {
                R::<String>{
                    code: 400,
                    msg: Option::from(e.to_string()),
                    data: None,
                }
            }
        }
}

pub async fn api_email_captcha_check(
    session: Session, 
    dto: web::Json<UserCaptchaEmailCheck>
)
    -> impl Responder
{
    let captcha = session.get::<String>(CAPTCHA).unwrap();
    if captcha.is_none(){
        return R::<String>{
            code: 400,
            msg: Option::from("[Error] Captcha Expired".to_string()),
            data: None,
        }
    }
    if captcha.unwrap() == dto.code {
        session.insert(ALLOW_NEXT_KEY, true).ok();
        R::<String> {
            code: 200,
            msg: Option::from("[Ok]".to_string()),
            data: None,
        }
    } else {
        R::<String> {
            code: 400,
            msg: Option::from("[Error] Captcha Error".to_string()),
            data: None,
        }
    }
}