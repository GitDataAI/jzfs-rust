use rbatis::RBatis;
pub mod login;
pub mod apply;
pub mod check;
pub mod reset;
pub mod update;
pub mod search;

#[derive(Clone)]
pub struct UserService{
    pub(crate) db: RBatis
}