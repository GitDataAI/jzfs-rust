use actix_web::{web, Responder};
use crate::api::app_write::AppWrite;
use crate::api::dto::repo_dto::RepoSearch;
use crate::api::dto::user_dto::UserOv;
use crate::metadata::service::MetaService;

#[utoipa::path(
    get,
    tag = "users",
    path = "/api/v1/users/search",
    params(
        ("keywords" = String, Query, description = "Search Keywords"),
        ("page" = u64, Query, description = "Page"),
        ("size" = u64, Query, description = "Size"),
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request")
    )
)]
pub async fn api_users_search(
    service: web::Data<MetaService>,
    query: web::Query<RepoSearch>
)
 -> impl Responder
{
    let (keywords, page, size) = (query.keywords.clone(), query.page, query.size);
    match service.user_service().search(keywords,page,size).await{
        Ok(data) => {
            AppWrite::<Vec<UserOv>>::ok(data)
        }
        Err(e) => {
            AppWrite::error(e.to_string())
        }
    }
}