use crate::http::{GitPack, GIT_ROOT};
use crate::services::AppState;

use std::path::PathBuf;
use std::process::Stdio;
use actix_web::{HttpMessage, HttpRequest, HttpResponseBuilder, Responder};
use actix_web::http::header::Header;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use tokio::process::Command;
use tracing::info;


pub async fn refs(
    request: HttpRequest,
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
    let version = request.headers().get("Git-Protocol").and_then(|x| x.to_str().ok());
    let mut response = HttpResponseBuilder::new(StatusCode::OK);
    response
        .insert_header(("Pragma", "no-cache"))
        .insert_header(("Cache-Control", "no-cache, max-age=0, must-revalidate"))
        .insert_header(("Expires", "Fri, 01 Jan 1980 00:00:00 GMT"));
    let url = request.uri().to_string().split("/")
        .map(|x| x.replace("/", ""))
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let mut cmd = Command::new("git");

    let server = if url.iter().any(|x| x.contains("git-upload-pack")) {
        response.insert_header(("Content-Type", "application/x-git-upload-pack-advertisement"));
        cmd.arg("upload-pack");
        GitPack::UploadPack
    } else if url.iter().any(|x| x.contains("git-receive-pack")) {
        response.insert_header(("Content-Type", "application/x-git-receive-pack-advertisement"));
        cmd.arg("receive-pack");
        
        GitPack::ReceivePack
    } else {
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body("Protoc Not Support");
    };
    
    let repo = repo.replace(".git", "");
    info!("repository ops: {}", format!("{}/{}", owner, repo));
    let repo = match status.repo_info(owner, repo).await {
        Ok(repo) => repo,
        Err(_) => return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
            .body("Repo Not Found"),
    };

    let path = PathBuf::from(format!("{}/{}/{}/", GIT_ROOT, repo.node_uid, repo.uid));

    if !path.exists() {
        return HttpResponseBuilder::new(StatusCode::NOT_FOUND)
            .body("repository not found");
    }

    cmd.arg("--stateless-rpc");
    cmd.arg("--advertise-refs");
    cmd.arg(".");
    cmd.current_dir(path);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    if let Some(version) = version {
        cmd.env("GIT_PROTOCOL", version);
    }

    let output = match cmd.output().await {
        Ok(output) => {
            info!("Command status: {:?}", output.status);
            output
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return HttpResponseBuilder::new(StatusCode::BAD_REQUEST)
                .body(e.to_string())
        }
    };

    let mut result = String::new();

    match server {
        GitPack::UploadPack => {
            result.push_str("001e# service=git-upload-pack\n");
            result.push_str("0000");
        }
        GitPack::ReceivePack => {
            result.push_str("001f# service=git-receive-pack\n");
            result.push_str("0000");
        }
    };

    result.push_str(std::str::from_utf8(&output.stdout).unwrap());
    response
        .body(result)
}
