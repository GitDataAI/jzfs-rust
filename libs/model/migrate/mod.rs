use async_trait::async_trait;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod users;
mod repos;

pub struct DatabaseMigrate;
#[async_trait]
impl MigratorTrait for DatabaseMigrate {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(users::UsersMigration),
            Box::new(repos::ReposMigration)
        ]
    }
}