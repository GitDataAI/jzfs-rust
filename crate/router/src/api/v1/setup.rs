use actix_session::Session;
use jzfs_module::entity::config::session::ALLOWNEXT;

pub async fn setup(session: Session){
    let is_login = session.get::<bool>(ALLOWNEXT).unwrap();
    if is_login.is_none() {
        session.insert(ALLOWNEXT, false).unwrap();
    }
    if !is_login.unwrap() {
        session.insert(ALLOWNEXT, true).unwrap();
    }
    return;
}