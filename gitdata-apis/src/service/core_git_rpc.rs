use gitdata::config::rpc::RpcConfig;
use gitdata::rpc::core_git::rep_repository_client::RepRepositoryClient;
use tokio::sync::OnceCell;

pub static RPC_CLIENT : OnceCell<CoreGitRpc> = OnceCell::const_new();


#[derive(Clone,Debug)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rpc() {
        let rpc = CoreGitRpc::get().await;
        dbg!(rpc);
    }
}