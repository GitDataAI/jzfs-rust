use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplyDto{
    pub username: String,
    pub passwd: String,
    pub email: String,
}

