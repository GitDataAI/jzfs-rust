use actix_session::SessionExt;
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use actix_web::middleware::Next;
use serde_json::json;
use crate::db::model::users;
use crate::api::controller::session::UsersModel;

pub async fn session_guard_handler(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let session = req.get_session();
    let model = session.get::<users::Model>(UsersModel);
    if model.is_err() {
        let msg = format!("Error during local session: {:?}", model.unwrap_err());
        let resp = HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
                "code": 500,
                "msg": msg
        }));
        return Ok(ServiceResponse::new(req.request().clone(),resp))
    }
    let model = model?;
    if model.is_none() {
        let resp =  HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(json!({
                "code": 401,
                "msg": "User is not logged in"
        }));
        return Ok(ServiceResponse::new(req.request().clone(),resp))
    }
    next.call(req).await
}
