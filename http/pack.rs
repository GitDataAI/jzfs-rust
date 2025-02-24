use std::io::Write;
use std::io::Read;
use crate::http::{GitPack, GIT_ROOT};
use crate::services::AppState;
use flate2::bufread::GzDecoder;
use std::io;
use std::io::Cursor;
use std::process::Stdio;
use bytes::Bytes;
use std::process::Command;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use actix_web::http::header::Header;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use async_stream::stream;
use tracing::{error};
use crate::services::repo::sync::RepoSync;

pub async fn pack(
    request: HttpRequest,
    payload: Bytes,
    path: Path<(String, String)>,
    status: Data<AppState>,
) -> impl Responder {
    let (owner, repo) = path.into_inner();

    let auth = match Authorization::<Basic>::parse(&request){
        Ok(auth) => auth,
        Err(_) => {
            return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
                .insert_header(("WWW-Authenticate", "Basic realm=\"GitData\""))
                .body("Unauthorized")
        }
    };
    let schema = auth.into_scheme();
    let username = schema.user_id().to_string();
    let password = match schema.password(){
        Some(password) => password.to_string(),
        None => {
            return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
                .insert_header(("WWW-Authenticate", "Basic realm=\"GitData\""))
                .body("Unauthorized")
        }
    };
    let (user,token) = if let Ok(res) = status.self_token_find(username, password).await {
        let user = res.0;
        let token = res.1;
        (user,token)
    }else {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
            .insert_header(("WWW-Authenticate", "Basic realm=\"GitData\""))
            .body("Unauthorized")
    };
    if token.access == *"read" {
        return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
            .insert_header(("WWW-Authenticate", "Basic realm=\"GitData\""))
            .body("Unauthorized");
    }
    if let Ok(access) = status.user_access_owner(user.uid).await {
        if !access.iter().any(|x|x.repos.contains(&repo.replace(".git",""))){
            return HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
                .insert_header(("WWW-Authenticate", "Basic realm=\"GitData\""))
                .body("Unauthorized");
        }
    }
    
    let bytes = if let Some(zip) = request.headers().get("content-encoding") {
        if zip == "gzip" {
            let mut decoder = GzDecoder::new(Cursor::new(payload.clone()));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                    .body(e.to_string());
            }
            decoded_data
        } else {
             payload.to_vec()
        }
    }else {
         payload.to_vec()
    };

    let version = request.headers().get("Git-Protocol").and_then(|x| x.to_str().ok());

    let mut response = HttpResponse::build(StatusCode::OK);
    response
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"))
        .insert_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"))
        .insert_header(("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload"))
        .insert_header(("X-Frame-Options", "DENY"));
    let url = request.uri().path().split("/")
        .map(|x| x.replace("/", ""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut cmd = Command::new("git");

    let _server = if url.iter().any(|x| x.contains("git-upload-pack")) {
        response.insert_header(("Content-Type", "application/x-git-upload-pack-result"));
        cmd.arg("upload-pack");
        GitPack::UploadPack
    } else if url.iter().any(|x| x.contains("git-receive-pack")) {
        response.insert_header(("Content-Type", "application/x-git-receive-pack-result"));
        cmd.arg("receive-pack");
       
        GitPack::ReceivePack
    } else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body("Protoc Not Support");
    };

 
    let repo = repo.replace(".git", "");
    let repo = match status.repo_info(owner, repo).await {
        Ok(repo) => repo,
        Err(_) =>
            return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body("Repo Not Found"),
    };

    cmd.arg("--stateless-rpc");
    cmd.arg(".");
    let path = format!("{}/{}/{}/", GIT_ROOT, repo.node_uid, repo.uid);
    cmd.current_dir(path);
    if let Some(version) = version {
        cmd.env("GIT_PROTOCOL", version);
    }
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                .body(e.to_string());
        }
    };

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let _stderr = child.stderr.take().unwrap();

    if let Err(e) = stdin.write_all(&bytes) {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body(e.to_string());
    }
    drop(stdin);

    let body = actix_web::body::BodyStream::new(stream! {
        let mut buffer = [0; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => {
                    break;
                }
                Ok(n) => {
                    yield Ok::<_, io::Error>(Bytes::copy_from_slice(&buffer[..n]));
                }
                Err(e) => {
                    error!("Error reading stdout: {}", e);
                    break;
                }
            }
        }
    });
    RepoSync::send(repo.uid).await;
    response
        .body(body)
}
