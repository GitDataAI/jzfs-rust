use crate::service::AppState;
use gitdata::config::rpc::RpcConfig;
use gitdata::rpc;
use gitdata::rpc::core_git::rep_repository_client::RepRepositoryClient;
use gitdata::rpc::git_core::{AccessResponse, PathRequest, PublickeyRequest, RepositoryAccess, RepositoryStoragePosition, TokenRequest};
use tokio::sync::OnceCell;
use tonic::{async_trait, Request, Response, Status};

pub static RPC_CLIENT : OnceCell<CoreGitRpc> = OnceCell::const_new();

#[derive(Clone, Debug)]
pub struct CoreGitRpc {
    pub client : RepRepositoryClient<tonic::transport::Channel>,
}

impl CoreGitRpc {
    pub async fn get() -> anyhow::Result<&'static CoreGitRpc> {
        RPC_CLIENT.get_or_try_init(Self::connect).await
    }
    async fn connect() -> anyhow::Result<Self> {
        let rpc = RpcConfig::load();
        if rpc.is_err() {
            return Err(anyhow::anyhow!("Failed to load rpc config"));
        }
        let rpc = rpc?;
        let core_git_rpc = RepRepositoryClient::connect(rpc.coregit_node()?.url()).await?;
        Ok(CoreGitRpc {
            client : core_git_rpc,
        })
    }
}



pub struct GitCoreRpc {
    state: AppState
}

impl GitCoreRpc {
    pub fn new(app_state: AppState) -> Self {
        Self {
            state: app_state
        }    
    }
}

#[async_trait]
impl rpc::git_core::rep_repository_server::RepRepository for GitCoreRpc {
    async fn path(&self, request: Request<PathRequest>) -> Result<Response<RepositoryStoragePosition>, Status> {
        let request = request.into_inner();
        let path = match self.state.repo_owner_name(request.owner,request.repo).await{
            Ok(path) => path,
            Err(_) => return Err(Status::not_found("Repo not found"))
        };
        return Ok(Response::new(RepositoryStoragePosition{
            path: path.uid.to_string(),
            node: path.storage_node,
        }))
    }

    async fn token(&self, request: Request<TokenRequest>) -> Result<Response<AccessResponse>, Status> {
        let request = request.into_inner();
        let model = match self.state.repo_owner_name(request.owner,request.repo).await{
            Ok(path) => path,
            Err(_) => return Err(Status::not_found("Repo not found"))
        };
        let tokens = match self.state.repo_access_token(model,request.token).await{
            Ok(tokens) => tokens,
            Err(_) => return Err(Status::not_found("Token not found"))
        };
        match tokens.access {
            1 => Ok(Response::new(AccessResponse{
                access: i32::from(RepositoryAccess::Read),
            })),
            4 => Ok(Response::new(AccessResponse{
                access: i32::from(RepositoryAccess::Write),
            })),
            7 => Ok(Response::new(AccessResponse{
                access: i32::from(RepositoryAccess::Admin),
            })),
            _ => Ok(Response::new(AccessResponse{
                access: i32::from(RepositoryAccess::None),
            }))
        }
    }

    async fn publickey(&self, request: Request<PublickeyRequest>) -> Result<Response<AccessResponse>, Status> {
        let request = request.into_inner();
        let model = match self.state.repo_owner_name(request.owner,request.repo).await{
            Ok(path) => path,
            Err(_) => return Err(Status::not_found("Repo not found"))
        };
        match self.state.repo_access_ssh(model,request.publickey).await{
            Ok(_) =>  Ok(Response::new(AccessResponse{
                access: i32::from(RepositoryAccess::Admin),
            })),
            Err(_) => Err(Status::not_found("Token not found"))
        }
    }
}

