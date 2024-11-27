use actix_web::web;
use config::result::R;
use dto::group::GroupSearchByName;
use crate::service::Service;

pub async fn api_group_search_name(
    service: web::Data<Service>,
    dto: web::Json<GroupSearchByName>
)
    -> impl actix_web::Responder
{
    match service.group_service.search_by_name(dto.into_inner().name).await{
        Ok(data) => {
            R::<Vec<model::groups::groups::GroupModel>>{
                code: 200,
                msg: Option::from("[Ok]".to_string()),
                data: Some(data)
            }
        }
        Err(e) => {
            R::<Vec<model::groups::groups::GroupModel>>{
                code: 500,
                msg: Option::from(e.to_string()),
                data: None,
            }
        }
    }
}