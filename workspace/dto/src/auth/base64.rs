use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base64Inner{
    pub inner: String
}