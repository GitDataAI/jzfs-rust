use actix_session::Session;
use actix_web::Responder;
use jzfs_module::entity::config::r::R;

pub async fn logout(session: Session) -> impl Responder{
    session.purge();
    R::<String>{
        code: 200,
        msg: Option::from("[Ok]".to_string()),
        data: None,
    }
}