use actix_session::Session;
use actix_web::{web, Responder};
use jzfs_module::entity::{base64, serde_json};
use jzfs_module::entity::base64::Engine;
use jzfs_module::entity::common::Inner;
use jzfs_module::entity::config::r::R;
use jzfs_module::entity::config::session::{ISLOGIN, USER};
use jzfs_module::entity::dto::auth::{AuthEmailPassword, AuthRespDto, AuthUserNamePassword};
use jzfs_module::Module;

pub async fn auth_by_username_password(up: web::Json<Inner>, module: web::Data<Module>, session: Session) -> impl Responder{
    let inner = up.inner.clone();
    let data = match base64::prelude::BASE64_STANDARD.decode(inner){
        Ok(data) => data,
        Err(_) => return R::<AuthRespDto>{
            code: 403,
            msg: Option::from("[Service 10001] Decode Failed".to_string()),
            data: None,
        
        }
    };
    let up: AuthUserNamePassword = match serde_json::from_slice(data.as_slice()){
        Ok(data) => data,
        Err(_) => return R::<AuthRespDto>{
            code: 403,
            msg: Option::from("[Service 10001] Decode Failed".to_string()),
            data: None,
        
        }
    };
    match module.auth_pwd(up).await{
        Ok(token) => {
            session.insert(USER.to_string(), token.clone()).ok();
            session.insert(ISLOGIN,true).ok();
            R::<AuthRespDto>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Option::from(token),
            }
        },
        Err(e) =>{
            R::<AuthRespDto>{
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}

pub async fn auth_by_email_password(ep: web::Json<Inner>, module: web::Data<Module>, session: Session) -> impl Responder{
    let inner = ep.inner.clone();
    let data = match base64::prelude::BASE64_STANDARD.decode(inner){
        Ok(data) => data,
        Err(_) => return R::<AuthRespDto>{
            code: 403,
            msg: Option::from("[Service 10001] Decode Failed".to_string()),
            data: None,

        }
    };
    let up: AuthEmailPassword = match serde_json::from_slice(data.as_slice()){
        Ok(data) => data,
        Err(_) => return R::<AuthRespDto>{
            code: 403,
            msg: Option::from("[Service 10001] Decode Failed".to_string()),
            data: None,

        }
    };
    match module.auth_email(up).await{
        Ok(token) => {
            session.insert(USER.to_string(), token.clone()).ok();
            session.insert(ISLOGIN,true).ok();
            R::<AuthRespDto>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Option::from(token),
            }
        },
        Err(e) =>{
            R::<AuthRespDto>{
                code: 403,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}