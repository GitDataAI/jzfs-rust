use crate::api::controller::session::{IsLogin, UsersModel};
use crate::api::server::auth::AuthServer;
use crate::api::server::email::{EmailMsg, EMAIL};
use crate::db::model::users;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use base64::engine::general_purpose::STANDARD;
use base64::prelude::*;
use lettre::message::Mailbox;
use lettre::Address;
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use std::ops::Add;
use std::time::Duration;
use time::{format_description, OffsetDateTime};
use uuid::Uuid;

// Must Base64
#[derive(Deserialize)]
pub struct LoginBean{
    pub username: String,
    pub password: String,
}
// Must Base64
#[derive(Deserialize)]
pub struct RegisterBean{
    pub username: String,
    pub password: String,
    pub email: String,
}
// Must Base64
#[derive(Deserialize)]
pub struct UpdatePassword{
    pub password: String,
}
#[derive(Deserialize)]
pub struct UpdateAny{
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub links: Option<Vec<String>>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub groups: Option<Vec<Uuid>>,
}
#[derive(Deserialize)]
pub struct SendEmail{
    pub email: String
}
#[derive(Deserialize)]
pub struct SendEmailCode{
    pub code: String
}
#[derive(Deserialize)]
pub struct AuthBase64{
    inner: String
}
#[inline]
pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg
        .route("login", web::post().to(login))
        .route("register", web::post().to(register))
        .route("logout", web::post().to(logout))
        .route("local", web::post().to(local))
        .route("updatawp",web::post().to(update_passwd))
        .route("updata",web::post().to(update_any))
        .route("send",web::post().to(send_code))
        .route("verification",web::post().to(verification_code))
    ;
}
#[inline]
pub async fn login(bean: web::Json<AuthBase64>,session: Session) -> impl Responder{
    let inner = STANDARD.decode(bean.inner.as_bytes());
    if inner.is_err() {
        return HttpResponse::Accepted()
            .json(json!({
                "code": 202,
                "msg": "Invalid base64"
            }))
    }
    let bean = serde_json::from_slice::<LoginBean>(&inner.unwrap());
    if bean.is_err() {
        return HttpResponse::InternalServerError()
        .json(json!({
            "code": 500,
            "msg": "Internal server error"
        }))
    }
    let bean = bean.unwrap();
    let server = AuthServer::new();
    match server.login(bean.username, bean.password).await {
        Ok(model) => {
            let format = format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
            ).unwrap();
            let create_at = model.create_at.format(&format).unwrap();
            let update_at = model.update_at.format(&format).unwrap();
            session.insert(UsersModel,model.clone()).unwrap();
            session.insert(IsLogin,true).unwrap();
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Login successful",
                "data":{
                    "uid": model.uid,
                    "name": model.name,
                    "username": model.username,
                    "email": model.email,
                    "avatar": model.avatar_url,
                    "bio": model.bio,
                    "links": model.links,
                    "location": model.location,
                    "time_zone": model.time_zone,
                    "language": model.language,
                    "groups": model.groups,
                    "create_at": create_at,
                    "update_at": update_at,
                }
            }))
        }
        Err(e) => {
            let msg = format!("Error during login: {:?}", e);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg,
            }))
        }
    }
}
#[inline]
pub async fn register(session: Session,bean: web::Json<AuthBase64>) -> impl Responder{
    let inner = STANDARD.decode(bean.inner.as_bytes());
    if inner.is_err() {
        return HttpResponse::Accepted()
            .json(json!({
                "code": 202,
                "msg": "Invalid base64"
            }))
    }
    let bean = serde_json::from_slice::<RegisterBean>(&inner.unwrap());
    if bean.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({
            "code": 500,
            "msg": "Internal server error"
        }))
    }
    let _ = match session.get::<bool>("next").unwrap(){
        None => {
            return HttpResponse::Ok()
                .json(json!({
                    "code": 400,
                    "msg": "please send verification_code"
                }))
        }
        Some(code) => {
            code
        }
    };
    let bean = bean.unwrap();
    let server = AuthServer::new();
    match server.register(bean.username,bean.password,bean.email).await {
        Ok(_model) => {
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Register successful",
            }))
        }
        Err(e) => {
            let msg = format!("Error during register: {:?}", e);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg,
            }))
        }
    }
}
#[inline]
pub async fn logout(session: Session) -> impl Responder{
    session.clear();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "code": 200,
            "msg": "Logout successful"
    }))
}
#[inline]
pub async fn local(session: Session) -> impl Responder{
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        return HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
            "code": 500,
            "msg": msg
        }))
    }
    let model = model.unwrap();
    if model.is_none() {
        return HttpResponse::Unauthorized()
        .content_type("application/json")
        .json(json!({
            "code": 401,
            "msg": "User is not logged in"
        }))
    }
    let model = model.unwrap();
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
    ).unwrap();
    let create_at = model.create_at.format(&format).unwrap();
    let update_at = model.update_at.format(&format).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
                "code": 200,
                "msg": "Login successful",
                "data":{
                    "uid": model.uid,
                    "name": model.name,
                    "username": model.username,
                    "email": model.email,
                    "avatar": model.avatar_url,
                    "bio": model.bio,
                    "links": model.links,
                    "location": model.location,
                    "time_zone": model.time_zone,
                    "language": model.language,
                    "groups": model.groups,
                    "create_at": create_at,
                    "update_at": update_at,
                }
            }
        )
    )


}
#[inline]
pub async fn update_passwd(bean: web::Json<AuthBase64>,session: Session) -> impl Responder{
    let inner = STANDARD.decode(bean.inner.as_bytes());
    if inner.is_err() {
        return HttpResponse::Accepted()
            .json(json!({
                "code": 202,
                "msg": "Invalid base64"
            }))
    }
    let bean = serde_json::from_slice::<UpdatePassword>(&inner.unwrap());
    if bean.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({
            "code": 500,
            "msg": "Internal server error"
        }))
    }
    let bean = bean.unwrap();
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
            "code": 500,
            "msg": msg
        }))
    }
    let model = model.unwrap();
    if model.is_none() {
        return HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(json!({
            "code": 401,
            "msg": "User is not logged in"
        }))
    }
    let model = model.unwrap();
    match AuthServer::new().reset_passwd(model.uid,bean.password).await{
        Ok(_) => {
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Reset PassWd Successful",
            }))
        }
        Err(e) => {
            let msg = format!("Error during reset passwd: {:?}", e);
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg,
            }))
        }
    }

}
#[inline]
pub async fn update_any(bean: web::Json<UpdateAny>,session: Session) -> impl Responder{
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
            "code": 500,
            "msg": msg
        }))
    }
    let model = model.unwrap();
    if model.is_none() {
        return HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(json!({
            "code": 401,
            "msg": "User is not logged in"
        }))
    }
    let model = model.unwrap();
    match AuthServer::new().update(model.uid,bean.0).await {
        Ok(model) => {
            let format = format_description::parse(
                "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
            ).unwrap();
            let create_at = model.create_at.format(&format).unwrap();
            let update_at = model.update_at.format(&format).unwrap();
            session.insert(UsersModel,model.clone()).unwrap();
            HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 200,
                "msg": "Login successful",
                 "data":{
                    "uid": model.uid,
                    "name": model.name,
                    "username": model.username,
                    "email": model.email,
                    "avatar": model.avatar_url,
                    "bio": model.bio,
                    "links": model.links,
                    "location": model.location,
                    "time_zone": model.time_zone,
                    "language": model.language,
                    "groups": model.groups,
                    "create_at": create_at,
                    "update_at": update_at,
                }
            }))
        }
        Err(e) => {
            let msg = format!("Error during update: {:?}", e);
            HttpResponse::Ok()
                .json(json!({
                    "code": 500,
                    "msg": msg
                }))
        }
    }
}
#[inline]
pub async fn send_code(session: Session, bean: web::Json<SendEmail>) -> impl Responder{
    let mut rng = rand::thread_rng();
    let verification_code: String = (0..6)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect();
    let address = match bean.email.parse::<Address>(){
        Ok(data) => {
            data
        }
        Err(err) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                    "code": 400,
                    "msg": err.to_string()
                }))
        }
    };
    session.insert("verification_code", verification_code.clone()).ok();
    session.insert("verification_code_time", &OffsetDateTime::now_utc()).ok();
    let msg = EmailMsg{
        from: Mailbox::new(Some(String::from("GitData Bot")), "zy@gitdata.ai".parse().unwrap()),
        reply: Mailbox::new(Some(String::from("zhenyi@gmail.com")), "zhenyi@gmail.com".parse().unwrap()),
        to: Mailbox::new(None,address ),
        subject: "GitData Verification Code".to_string(),
        body: format!(r#"
            <!DOCTYPE html>
<html>
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
    <title></title>
    <meta charset="utf-8" />

</head>
<body>
    <div class="qmbox qm_con_body_content qqmail_webmail_only" id="mailContentContainer" style="">
        <style type="text/css">
            .qmbox body {{
                margin: 0;
                padding: 0;
                background: #fff;
                font-family: "Verdana, Arial, Helvetica, sans-serif";
                font-size: 14px;
                line-height: 24px;
            }}

            .qmbox div, .qmbox p, .qmbox span, .qmbox img {{
                margin: 0;
                padding: 0;
            }}

            .qmbox img {{
                border: none;
            }}

            .qmbox .contaner {{
                margin: 0 auto;
            }}

            .qmbox .title {{
                margin: 0 auto;
                background: url() #CCC repeat-x;
                height: 30px;
                text-align: center;
                font-weight: bold;
                padding-top: 12px;
                font-size: 16px;
            }}

            .qmbox .content {{
                margin: 4px;
            }}

            .qmbox .biaoti {{
                padding: 6px;
                color: #000;
            }}

            .qmbox .xtop, .qmbox .xbottom {{
                display: block;
                font-size: 1px;
            }}

            .qmbox .xb1, .qmbox .xb2, .qmbox .xb3, .qmbox .xb4 {{
                display: block;
                overflow: hidden;
            }}

            .qmbox .xb1, .qmbox .xb2, .qmbox .xb3 {{
                height: 1px;
            }}

            .qmbox .xb2, .qmbox .xb3, .qmbox .xb4 {{
                border-left: 1px solid #BCBCBC;
                border-right: 1px solid #BCBCBC;
            }}

            .qmbox .xb1 {{
                margin: 0 5px;
                background: #BCBCBC;
            }}

            .qmbox .xb2 {{
                margin: 0 3px;
                border-width: 0 2px;
            }}

            .qmbox .xb3 {{
                margin: 0 2px;
            }}

            .qmbox .xb4 {{
                height: 2px;
                margin: 0 1px;
            }}

            .qmbox .xboxcontent {{
                display: block;
                border: 0 solid #BCBCBC;
                border-width: 0 1px;
            }}

            .qmbox .line {{
                margin-top: 6px;
                border-top: 1px dashed #B9B9B9;
                padding: 4px;
            }}

            .qmbox .neirong {{
                padding: 6px;
                color: #666666;
            }}

            .qmbox .foot {{
                padding: 6px;
                color: #777;
            }}

            .qmbox .font_darkblue {{
                color: #006699;
                font-weight: bold;
            }}

            .qmbox .font_lightblue {{
                color: #008BD1;
                font-weight: bold;
            }}

            .qmbox .font_gray {{
                color: #888;
                font-size: 12px;
            }}
        </style>
        <div class="contaner">
            <div class="title">GitData Captcha</div>
            <div class="content">
                <p class="biaoti"><b>Hello, Dear user!</b></p>
                <b class="xtop"><b class="xb1"></b><b class="xb2"></b><b class="xb3"></b><b class="xb4"></b></b>
                <div class="xboxcontent">
                    <div class="neirong">
                        <p><b>Your verification code：</b><span class="font_lightblue"><span id="yzm" onclick="return false;" t="7" style="border-bottom: 1px dashed rgb(204, 204, 204); z-index: 1; position: static;">{}</span></span><br><span class="font_gray">(请输入该验证码完成验证，验证码30分钟内有效！)</span></p>
                        <div class="line">If you have not applied, please ignore this email.</div>
                    </div>
                </div>
                <b class="xbottom"><b class="xb4"></b><b class="xb3"></b><b class="xb2"></b><b class="xb1"></b></b>
            </div>
        </div>
        <style type="text/css">
            .qmbox style, .qmbox script, .qmbox head, .qmbox link, .qmbox meta {{
                display: none !important;
            }}
        </style>
    </div>
</body>
</html>
        "#,verification_code)
    };
    match EMAIL.get().unwrap().send(msg){
        Ok(_) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 200,
                    "msg": "success"
                }))
        }
        Err(_) => {
            HttpResponse::Ok()
                .json(json!({
                    "code": 300,
                    "msg": "failed"
                }))
        }
    }
}
#[inline]
pub async fn verification_code(session: Session, code: web::Json<SendEmailCode>) -> impl Responder{
    let verification_code = match session.get::<String>("verification_code").unwrap(){
        None => {
            return HttpResponse::Ok()
                .json(json!({
                    "code": 400,
                    "msg": "failed 03"
                }))
        }
        Some(code) => {
            code
        }
    };
    let verification_code_time = match session.get::<OffsetDateTime>("verification_code_time").unwrap(){
        None => {
            return HttpResponse::Ok()
                .json(json!({
                    "code": 400,
                    "msg": "failed 02"
                }))
        }
        Some(code) => {
            code
        }
    };
    if verification_code_time.add(Duration::from_millis(30)) > OffsetDateTime::now_utc(){
        return HttpResponse::Ok()
            .json(json!({
                "code": 400,
                "msg": "failed 01"
            }))
    }
    if verification_code == code.code {
        session.remove("verification_code");
        session.remove("verification_code_time");
        session.insert("next",true).ok();
        HttpResponse::Ok()
            .json(json!({
                "code": 200,
                "msg": "success"
            }))
    }else {
        HttpResponse::Ok()
            .json(json!({
                "code": 400,
                "msg": "failed 04"
            }))
    }

}