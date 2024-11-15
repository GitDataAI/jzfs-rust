#![allow(unused)]

use crate::db::auth_db::AUTHDB;
use crate::db::model::{public_key, public_token, users};
use crate::ssh::server::AuthType;
use russh::server::{Auth, Msg, Session};
use russh::{Channel, ChannelId, MethodSet};
use russh_keys::key::PublicKey;
use russh_keys::PublicKeyBase64;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::io::Write;
use std::process::ChildStdin;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;

pub struct SSHSession {
    session: Arc<Mutex<HashMap<(String,AuthType),users::Model>>>,
    clients: Arc<Mutex<HashMap<(String, AuthType), (ChannelId,Channel<Msg>)>>>,
    stdin: HashMap<ChannelId, ChildStdin>,
    db: DatabaseConnection
}

impl Default for SSHSession {
    fn default() -> Self {
        let db = AUTHDB.get().unwrap().clone();
        let session = Arc::new(Mutex::new(HashMap::new()));
        let clients = Arc::new(Mutex::new(HashMap::new()));
        let stdin = HashMap::new();
        Self { session, clients, stdin, db }
    }
}

impl SSHSession {

}

#[async_trait]
impl russh::server::Handler for SSHSession {
    type Error = russh::Error;
    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        if user != "git"{
            return Ok(Auth::Reject {
                proceed_with_methods: None
            })
        }
        self.auth_reject_pubkey()
    }
    async fn auth_password(&mut self, user: &str, password: &str) -> Result<Auth, Self::Error> {
        if user != "git"{
            return Ok(Auth::Reject {
                proceed_with_methods: None
            })
        }
        let model = public_token::Entity::find()
            .filter(public_key::Column::PublicKey.eq(password.to_string()))
            .one(&self.db)
            .await;
        if model.is_err() {
            return Err(Self::Error::Disconnect)
        }
        let model = model.unwrap();
        if model.is_none(){
            return Ok(Auth::Reject {
                proceed_with_methods: Some(MethodSet::PASSWORD),
            });
        }
        let model = model.unwrap();
        let user_id = model.user_id;
        let user_model = users::Entity::find()
            .filter(users::Column::Uid.eq(user_id))
            .one(&self.db)
            .await.unwrap();
        if user_model.is_none() {
            return Err(Self::Error::NoHomeDir)
        }
        let user_model = user_model.unwrap();
        let mut r = self.session.lock().unwrap();
        r.insert((password.to_string(),AuthType::Token),user_model);
        Ok(Auth::Accept)
    }
    async fn auth_publickey(&mut self, user: &str, pubkey: &PublicKey) -> Result<Auth, Self::Error> {
        if user != "git"{
            return Ok(Auth::Reject {
                    proceed_with_methods: None
                })
        }
        let model = public_key::Entity::find()
            .filter(public_key::Column::PublicKey.eq(pubkey.public_key_base64()))
            .one(&self.db)
            .await;
        if model.is_err() {
            return Err(Self::Error::Disconnect)
        }
        let model = model.unwrap();
        if model.is_none(){
            return Ok(Auth::Reject {
                    proceed_with_methods: Some(MethodSet::PASSWORD),
                });
        }
        let model = model.unwrap();
        let user_id = model.user_id;
        let user_model = users::Entity::find()
            .filter(users::Column::Uid.eq(user_id))
            .one(&self.db)
            .await.unwrap();
        if user_model.is_none() {
            return Err(Self::Error::NoHomeDir)
        }
        let user_model = user_model.unwrap();
        let mut r = self.session.lock().unwrap();
        r.insert((pubkey.public_key_base64(),AuthType::PublicKey),user_model);
        Ok(Auth::Accept)
    }
    async fn channel_eof(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Self::Error> {
        let stdin = self.stdin.remove(&channel);
        if let Some(mut stdin) = stdin {
            stdin.flush()?;
        }
        session.flush().ok();
        session.close(channel);
        Ok(())
    }
    async fn data(&mut self, channel: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
        self.send_stdin(channel,data).await?;
        Ok(())
    }
    async fn exec_request(&mut self, channel: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
        // let git_shell_cmd = std::str::from_utf8(data)?;
        // let (service, path) = parse_git_shell_cmd(git_shell_cmd);
        todo!()
    }
}

impl SSHSession {
    fn auth_reject_pubkey(&self) -> Result<Auth, russh::Error> {
        Ok(
            Auth::Reject {
                proceed_with_methods: Some(MethodSet::PUBLICKEY),
            },
        )
    }
    async fn send_stdin(
        &mut self,
        channel_id: ChannelId,
        data: &[u8],
    ) -> Result<(), russh::Error> {
        if let Some(stdin) = self.stdin.get_mut(&channel_id) {
            stdin.write_all(data)?;
        }
        Ok(())
    }
}