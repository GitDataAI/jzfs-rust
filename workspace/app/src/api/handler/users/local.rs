use actix_session::Session;
use actix_web::Responder;
use config::result::R;
use dto::session::{SessionUserValue, SESSION_USER_KEY};

pub async fn api_user_local(
    session: Session
)
-> impl Responder{
    let model = session.get::<SessionUserValue>(SESSION_USER_KEY).unwrap().unwrap();
    R::<SessionUserValue>{
        code: 200,
        msg: Option::from("[Ok]".to_string()),
        data: Some(model),
    }
}