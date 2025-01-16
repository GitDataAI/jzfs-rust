use gitdata::config::database::DatabaseConfig;

fn main() {
    DatabaseConfig::default().write().unwrap();
}
