use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{HttpRequest, HttpResponse, Responder, web};

use crate::mount::{StoragePool, StorageSingleton};
use crate::rpc::RepRepository;
use crate::rpc::repository::RepositoryRpc;

pub(crate) async fn text(
    request: HttpRequest,
    path: web::Path<(String, String)>,
    rpc: web::Data<RepositoryRpc>,
    storage: web::Data<StoragePool>,
) -> impl Responder {
    let (owner, path) = path.into_inner();
    let nodepath = if let Ok(repo) = rpc.path(owner.clone(), path.clone()).await {
        repo
    } else {
        return HttpResponse::NotFound().finish();
    };
    let file_path = request
        .uri()
        .to_string()
        .replace(&format!("{}/{}", owner, path), "");
    let storage = match match storage.get(nodepath.clone()) {
        Some(storage) => storage,
        None => return HttpResponse::NotFound().finish(),
    } {
        StorageSingleton::S3(x) => x.text(&nodepath.path(), &file_path).await,
        StorageSingleton::Local(x) => x.text(&nodepath.path(), &file_path).await,
        StorageSingleton::Nfs(x) => x.text(&nodepath.path(), &file_path).await,
    };
    if storage.is_err() {
        return HttpResponse::NotFound().finish();
    }
    let storage = storage.unwrap();
    let namedfile = storage.use_last_modified(true);
    let mut response = namedfile.into_response(&request);
    response.headers_mut().insert(
        HeaderName::try_from("Pragma".to_string()).unwrap(),
        HeaderValue::try_from("no-cache".to_string()).unwrap(),
    );
    response.headers_mut().insert(
        HeaderName::try_from("Cache-Control".to_string()).unwrap(),
        HeaderValue::try_from("no-cache, max-age=0, must-revalidate".to_string()).unwrap(),
    );
    response.headers_mut().insert(
        HeaderName::try_from("Expires".to_string()).unwrap(),
        HeaderValue::try_from("Fri, 01 Jan 1980 00:00:00 GMT".to_string()).unwrap(),
    );
    if request.uri().to_string().contains("pack") {
        if request.uri().to_string().ends_with(".pack") {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::try_from("application/x-git-packed-objects".to_string()).unwrap(),
            );
        } else if request.uri().to_string().ends_with(".idx") {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::try_from("application/x-git-packed-objects-toc".to_string()).unwrap(),
            );
        } else {
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::try_from("application/x-git-loose-object".to_string()).unwrap(),
            );
        }
    } else if request.uri().to_string().contains("objects/info/packs") {
        let time = chrono::Local::now();
        let expires = time + chrono::Duration::days(1);
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::try_from("application/x-git-loose-object".to_string()).unwrap(),
        );
        response.headers_mut().insert(
            HeaderName::try_from("Date".to_string()).unwrap(),
            HeaderValue::try_from(time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()).unwrap(),
        );
        response.headers_mut().insert(
            HeaderName::try_from("Expires".to_string()).unwrap(),
            HeaderValue::try_from(expires.format("%a, %d %b %Y %H:%M:%S GMT").to_string()).unwrap(),
        );
        response.headers_mut().insert(
            HeaderName::try_from("Cache-Control".to_string()).unwrap(),
            HeaderValue::try_from("public, max-age=86400".to_string()).unwrap(),
        );
    }
    response
}
