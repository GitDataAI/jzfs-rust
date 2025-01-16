use crate::service::AppState;
use apalis::prelude::Data;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone,Deserialize,Serialize,Debug)]
pub struct SyncRepoMessage {
    pub node: String,
    pub path: String,
}

pub(crate) async fn sync_repo(
    message: SyncRepoMessage,
    data: Data<AppState>,
) -> io::Result<()> {
    SyncRepo{
        node: message.node,
        path: message.path,
        state: data,
    }.run().await
        .map_err(|x| {
            io::Error::new(io::ErrorKind::Other, x)
        })?;
    Ok(())
}

pub struct SyncRepo {
    pub node: String,
    pub path: String,
    pub state: Data<AppState>,
}


impl SyncRepo {

    pub async fn run(self) -> anyhow::Result<()> {
        
        
        Ok(())
    }
}