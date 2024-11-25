use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct CommonOne<T>{
    pub data: T
}

#[derive(Deserialize)]
pub struct Inner{
    pub inner: String
}

