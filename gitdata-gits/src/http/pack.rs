use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::http::header::CONTENT_ENCODING;
use actix_web::web::Payload;
use actix_web::{HttpRequest, HttpResponseBuilder, Responder, web};
use bytes::Bytes;
use futures::StreamExt;

use crate::mount::{StoragePool, StorageSingleton};
use crate::rpc::RepRepository;
use crate::rpc::repository::RepositoryRpc;
use crate::service::GitServiceType;

pub async fn pack(
    request: HttpRequest,
    path: web::Path<(String, String)>,
    mut payload: Payload,
    rpc: web::Data<RepositoryRpc>,
    storage: web::Data<RwLock<StoragePool>>,
) -> impl Responder {
    let (owner, repo_name) = path.into_inner();
    let repo_name = repo_name.replace(".git", "");
    let path = match rpc.path(owner, repo_name).await {
        Ok(path) => path,
        Err(_) => return actix_web::HttpResponse::NotFound().finish(),
    };
    let service = if request.uri().to_string().contains("receive-pack") {
        "git-receive-pack".to_string()
    } else if request.uri().to_string().contains("upload-pack") {
        "git-upload-pack".to_string()
    } else {
        return actix_web::HttpResponse::NotFound().finish();
    };

    // TODO Token Authenticate
    let version = request
        .headers()
        .get("Git-Protocol")
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");

    let service = match GitServiceType::from_string(&service) {
        Some(service) => service,
        None => return actix_web::HttpResponse::NotFound().finish(),
    };

    let mut bytes = Vec::new();
    while let Some(Ok(data)) = payload.next().await {
        bytes.extend_from_slice(&data);
    }
    let bytes = Bytes::from(bytes);

    let gzip = request
        .headers()
        .get(CONTENT_ENCODING)
        .map(|x| x.to_str().unwrap() == "gzip")
        .is_some();

    let mut response = HttpResponseBuilder::new(StatusCode::OK);
    response.append_header(("Content-Type", "application/x-git-upload-pack-advertise"));
    response.append_header(("Connection", "Keep-Alive"));
    response.append_header(("Transfer-Encoding", "chunked"));
    response.append_header(("X-Content-Type-Options", "nosniff"));
    let storage = storage.read().unwrap();
    match match storage.get(path.clone()) {
        Some(storage) => storage,
        None => {
            return actix_web::HttpResponse::NotFound().finish();
        }
    } {
        StorageSingleton::S3(s) => {
            match s
                .pack(
                    path.path().to_string(),
                    service,
                    Some(version.to_string()),
                    gzip,
                    bytes,
                )
                .await
            {
                Ok(bytes) => response.streaming(bytes),
                Err(_) => actix_web::HttpResponse::NotFound().finish(),
            }
        }
        StorageSingleton::Local(s) => {
            match s
                .pack(
                    path.path().to_string(),
                    service,
                    Some(version.to_string()),
                    gzip,
                    bytes,
                )
                .await
            {
                Ok(bytes) => response.streaming(bytes),
                Err(_) => actix_web::HttpResponse::NotFound().finish(),
            }
        }
        StorageSingleton::Nfs(s) => {
            match s
                .pack(
                    path.path().to_string(),
                    service,
                    Some(version.to_string()),
                    gzip,
                    bytes,
                )
                .await
            {
                Ok(bytes) => response.streaming(bytes),
                Err(_) => actix_web::HttpResponse::NotFound().finish(),
            }
        }
    }
}
