use rbatis::RBatis;
pub mod create;
pub mod check;
pub mod delete;
pub mod list;
pub mod update;
pub struct RepoService {
    pub db: RBatis,
}