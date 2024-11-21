use crate::server::{Backend, LB};
use k8s_openapi::api::core::v1::Endpoints;
use kube::api::ListParams;
use kube::{Api, Client, Config};
use pingora_load_balancing::LoadBalancer;
use pingora_proxy::http_proxy_service;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use kube::config::{KubeConfigOptions, Kubeconfig};
use tracing::info;

pub mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let kube = env::var("KUBE").unwrap();
    let kube_config:Kubeconfig = serde_yaml::from_str(&kube).unwrap();
    let config = Config::from_custom_kubeconfig(kube_config,&KubeConfigOptions::default()).await.unwrap();
    let client = Client::try_from(config)
        .unwrap();
    let pods: Api<Endpoints> = Api::namespaced(client, "gitdata");
    let item = pods.list(&ListParams::default()).await.unwrap();
    let mut api = vec![];
    let mut home = vec![];
    let mut cloud = vec![];
    for idx in item.items{
        let name = idx.metadata.name.unwrap();
        let host = idx.subsets.unwrap()
            .iter()
            .map(|x| x.clone().addresses.unwrap().iter().map(|x|x.ip.clone()).collect::<Vec<String>>())
            .collect::<Vec<_>>();
        match name.as_str() {
            "gitdata-ai-service"=>{
                for idx in  host{
                    for idx in idx{
                        home.push(idx.clone());
                        info!("Service: gitdata-ai-service   EnPrint:{}",idx )
                    }
                }
            },
            "gitdata-rust-service"=>{
                for idx in  host{
                    for idx in idx{
                        cloud.push(idx.clone());
                        info!("Service: gitdata-rust-service   EnPrint:{}",idx )
                    }
                }
            },
            "jzfs-rust-service"=>{
                for idx in  host{
                    for idx in idx{
                        api.push(idx.clone());
                        info!("Service: jzfs-rust-service   EnPrint:{}",idx )
                    }
                }
            }
            _=>{}
        }
    }
    let mut server = pingora::server::Server::new(None).unwrap();
    server.bootstrap();
    let api = LoadBalancer::try_from_iter(api)?;
    let cloud = LoadBalancer::try_from_iter(cloud)?;
    let home = LoadBalancer::try_from_iter(home)?;
    let mut hash = HashMap::new();
    hash.insert(Backend::Api,api);
    hash.insert(Backend::Cloud,cloud);
    hash.insert(Backend::Home,home);
    let mut lb = http_proxy_service(&server.configuration, LB(Arc::new(hash)));
    lb.add_tcp("0.0.0.0:80");
    server.add_service(lb);
    server.run_forever()
}
