use crate::server;
use crate::server::email::{init_email, EMAIL_SERVICE};
use crate::service::email::EmailService;
use crate::service::group::GroupService;
use crate::service::user::UserService;

pub mod auth;
pub mod user;
pub mod email;
pub mod group;
pub mod repo;

#[derive(Clone)]
pub struct Service{
    pub user_service: UserService,
    pub auth_service: auth::AuthService,
    pub email_service: EmailService,
    pub group_service: GroupService,
}

impl Service {
    pub async fn new() -> Service {
        let db = server::db::init().await;
        init_email().await;
        let service = Service{
            user_service: UserService {
                db: db.clone()
            },
            auth_service: auth::AuthService {
                _db: db.clone()
            },
            email_service: EmailService {
                db: db.clone(),
                email: EMAIL_SERVICE.get().unwrap().clone(),
            },
            group_service: GroupService {
                db: db.clone()
            },
        };
        service
    }
}