use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailCaptchaSend{
    pub email: String,
    pub language: Option<String>
}


#[derive(Deserialize)]
pub struct EmailCaptchaVerify{
    pub captcha: String
}