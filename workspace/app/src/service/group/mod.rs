use rbatis::RBatis;

pub mod create;
pub mod delete;
pub mod check;
pub mod update;
pub mod move_owner;
pub mod search;
pub mod list;

#[derive(Clone)]
pub struct GroupService{
    pub(crate) db: RBatis
}