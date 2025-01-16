use std::collections::HashMap;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::Responder;
use actix_web::http::StatusCode;
use actix_web::http::header::HeaderValue;
use actix_web::web;

use crate::mount::StoragePool;
use crate::rpc::git_core::RepositoryRpc;
use crate::service::GitServiceType;

pub async fn info_refs(
    req : HttpRequest,
    path : web::Path<(String, String)>,
    query : web::Query<HashMap<String, String>>,
    rpc : web::Data<RepositoryRpc>,
    storage : web::Data<StoragePool>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let path = match rpc.path(owner, repo_name).await {
        Ok(path) => path,
        Err(_) => {
            return HttpResponse::NotFound().body("Not found");
        }
    };
    let service_name = query.get("service").unwrap_or(&"".to_string()).to_string();
    if service_name != "git-receive-pack" && service_name != "git-upload-pack" {
        return HttpResponse::BadRequest().body("Invalid service name");
    }
    let service = match GitServiceType::from_string(service_name.as_str()) {
        Some(service) => service,
        None => {
            return HttpResponse::BadRequest().body("Invalid service name");
        }
    };
    let version = req
        .headers()
        .get("Git-Protocol")
        .unwrap_or(&HeaderValue::from_str("").unwrap())
        .to_str()
        .map(|s| s.to_string())
        .unwrap_or("".to_string());

    let storage = match storage.node.get(&path.clone().node) {
        Some(storage) => storage,
        None => {
            return HttpResponse::NotFound().body("Not found");
        }
    };
    let data = match storage
        .refs(&*path.path.clone(), service, Some(&version))
        .await
    {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::NotFound().body("Not found");
        }
    };
    let mut response = HttpResponseBuilder::new(StatusCode::OK);
    response.append_header((
        "Content-Type",
        format!("application/x-git-{}-advertisement", service_name),
    ));
    response.append_header(("Pragma", "no-cache"));
    response.append_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"));
    response.append_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"));
    let mut body = String::new();
    match service_name.as_str() {
        "upload-pack" => {
            body.push_str(&"001e# service=git-upload-pack\n".to_string());
            body.push_str("0000");
        }
        "receive-pack" => {
            body.push_str(&"001f# service=git-receive-pack\n".to_string());
            body.push_str("0000");
        }
        _ => {}
    }
    body.push_str(&data);
    response.body(body.as_bytes().to_vec())
}
