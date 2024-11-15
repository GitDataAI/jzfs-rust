use std::net::SocketAddr;
use crate::ssh::server::session::SSHSession;

#[derive(Clone)]
pub struct SSHServer;

impl russh::server::Server for SSHServer {
    type Handler = SSHSession;

    fn new_client(&mut self, _: Option<SocketAddr>) -> Self::Handler {
        SSHSession::default()
    }
}