use actix_session::Session;
use actix_web::Responder;
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::USER;

pub async fn local(session: Session) -> impl Responder{
    let user = session.get::<String>(USER).unwrap();
    if user.is_none() {
        return R::<String>{
            code: 403,
            msg: Option::from("[Service 10001] Permission denied".to_string()),
            data: None,
        }
    }
    R::<String>{
        code: 200,
        msg: Option::from("[Ok]".to_string()),
        data: user,
    }
}