use anyhow::anyhow;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::Deserialize;
use serde::Serialize;

pub mod passwd;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthInner {
    inner : String,
}

impl AuthInner {
    pub fn decode<T>(&self) -> anyhow::Result<T>
    where
        T : for<'de> Deserialize<'de>,
    {
        match BASE64_STANDARD.decode(self.inner.as_bytes()) {
            Ok(data) => match serde_json::from_slice::<T>(&data) {
                Ok(data) => Ok(data),
                Err(err) => Err(anyhow!("{}", err)),
            },
            Err(err) => Err(anyhow!("{}", err)),
        }
    }
}
