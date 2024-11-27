pub mod captcha;
pub mod forget;

use rbatis::RBatis;
use crate::server::email::EmailServer;


#[derive(Clone)]
pub struct EmailService{
    pub db: RBatis,
    pub email: EmailServer,
}