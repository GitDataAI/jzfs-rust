use serde::{Deserialize, Serialize};

pub const ALLOW_NEXT_KEY: &str = "allow_next";
pub const CAPTCHA: &str = "captcha";
pub const SESSION_USER_KEY: &str = "session_user";
#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct SessionUserValue{
    pub uid: rbatis::rbdc::Uuid,
    pub name: String,
    pub pro: bool,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub bio: String,
}