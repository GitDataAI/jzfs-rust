use actix_session::Session;
use actix_web::{web, Responder};
use rand::Rng;
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::{ALLOWNEXT, CAPTCHA};
use jzfs_module::entity::dto::email::EmailCaptchaSend;
use jzfs_module::Module;

pub async fn send(session: Session, email: web::Json<EmailCaptchaSend>, module: web::Data<Module>) -> impl Responder{
    let mut rng = rand::thread_rng(); 
    let random_number: u32 = rng.gen_range(100000..1000000);
    session.insert(CAPTCHA, random_number).ok();
    match module.email_send_captcha(email.email.clone(), random_number.to_string()).await{
        Ok(_) => {
            R::<String>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: None,
            }
        }
        Err(e) => {
            R::<String>{
                code: 503,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}

pub async fn check(session: Session, captcha: web::Json<String>) -> impl Responder{
    let captcha_session: Option<u32> = session.get(CAPTCHA).unwrap();
    match captcha_session{
        Some(captcha_session) => {
            if captcha_session == captcha.into_inner().parse::<u32>().unwrap(){
                session.insert(ALLOWNEXT, true).ok();
                return R::<String>{
                    code: 200,
                    msg: Option::from("[Ok]".to_string()),
                    data: None
                }
            }
        }
        None => {},
    }
    R::<String>{
        code: 403,
        msg: Option::from("[Service 10004] Captcha Error".to_string()),
        data: None
    }
}