// use std::sync::Arc;
// use std::time::Duration;
// use russh::server::Server;
// use russh_keys::key::KeyPair;
// use jzfs::db::auth_db;
// use jzfs::ssh::server::server::SSHServer;
// 
// #[tokio::main]
// async fn main(){
//     tracing_subscriber::fmt::init();
//     auth_db::init().await;
//     let config = russh::server::Config {
//         auth_rejection_time: Duration::from_secs(3),
//         auth_rejection_time_initial: Some(Duration::from_secs(0)),
//         keys: vec![
//             KeyPair::generate_ed25519()
//         ],
//         ..Default::default()
//     };
//     let mut server = SSHServer;
//     server.run_on_address(
//         Arc::new(config),
//         "0.0.0.0:2022"
//     ).await.unwrap();
// }
pub fn main() {
    println!("TODO")
}