pub mod server;
pub mod session;
pub mod init;
pub mod config;

#[derive(Eq, Hash, PartialEq)]
pub enum AuthType{
    Token,
    PublicKey,
}
