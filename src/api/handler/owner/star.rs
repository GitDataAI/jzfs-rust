use actix_session::Session;
use actix_web::web;
use uuid::Uuid;
use crate::api::service::Service;
use crate::utils::r::R;

#[utoipa::path(
    get,
    tag = "owner",
    path = "/api/v1/owner/star",
    responses(
            (status = 200, description = "OK", body = Vec<Uuid>),
            (status = 400, description = "User Not Login"),
            (status = 402, description = "User Not Exist"),
            (status = 403, description = "Get Star Failed"),
            (status = 405, description = "Other Error"),
    ),
)]
pub async fn api_owner_star(
    session: Session,
    service: web::Data<Service>
)
-> impl actix_web::Responder
{
    let model = service.check.check_session(session).await;
    if model.is_err(){
        return R::<Vec<Uuid>>{
            code: 400,
            msg: Option::from("[Error] User Not Login".to_string()),
            data: None,
        }
    }
    let model = model.unwrap();
    match service.users.star(model.uid).await{
        Ok(result)=>{
            R{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(result),
            }
        },
        Err(e)=>{
            R{
                code: 400,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}