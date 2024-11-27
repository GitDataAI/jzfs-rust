use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLogin{
    pub username: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserLoginEmail{
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserCaptchaEmail{
    pub email: String,
}
#[derive(Deserialize)]
pub struct UserCaptchaEmailCheck{
    pub email: String,
    pub code: String,
}
#[derive(Deserialize)]
pub struct UserApply{
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserResetPassword{
    pub email: String,
    pub token: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserUpdate{
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}
#[derive(Deserialize)]
pub struct UserResetPasswd {
    pub old_password: String,
    pub new_password: String
}
