use actix_session::Session;
use anyhow::anyhow;

use crate::service::users::info::UsersInfoReplay;

impl UsersInfoReplay {
    pub fn from_session(session : Session) -> anyhow::Result<Self> {
        let token = session.get::<Self>("token")?;
        if token.is_none() {
            return Err(anyhow!("token is none"));
        }
        Ok(token.unwrap())
    }
}
