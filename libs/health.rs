use crate::rpc::health::{HealthConfig, HealthCpu, HealthMemory};

impl HealthConfig {
    pub fn generate() -> HealthConfig {
        let mut sysinfo = sysinfo::System::new_all();
        sysinfo.refresh_cpu_all();
        let cpu = sysinfo
            .cpus()
            .iter()
            .map(|x| HealthCpu {
                name: x.name().to_string(),
                usage: x.cpu_usage(),
                frequency: x.frequency(),
                brand: x.brand().to_string(),
                vendor_id: x.vendor_id().to_string(),
            })
            .collect::<Vec<_>>();
        sysinfo.refresh_memory();
        let memory = HealthMemory {
            available_memory: sysinfo.available_memory(),
            free_memory: sysinfo.free_memory(),
            total_memory: sysinfo.total_memory(),
        };
        Self {
            cpu,
            memory: Some(memory),
        }
    }
}

pub mod client {
    use std::io;

    use log::warn;

    use crate::health::HealthConfig;
    use crate::rpc::health;

    pub struct HealthClients {
        client: health::health_client::HealthClient<tonic::transport::Channel>,
    }
    impl HealthClients {
        pub async fn new(addr: String) -> io::Result<Self> {
            let channel = tonic::transport::Channel::from_shared(addr)
                .map_err(|x| {
                    warn!("{}", x);
                    io::Error::new(io::ErrorKind::Other, "error")
                })?
                .connect()
                .await
                .map_err(|x| {
                    warn!("{}", x);
                    io::Error::new(io::ErrorKind::Other, "error")
                })?;
            Ok(Self {
                client: health::health_client::HealthClient::new(channel),
            })
        }
        pub async fn check(&mut self) -> Result<HealthConfig, tonic::Status> {
            let request = self
                .client
                .check(health::HealthCheckRequest {
                    service: "".to_string(),
                })
                .await?;
            Ok(request.into_inner())
        }
    }
}

pub mod service {
    use async_trait::async_trait;
    use tonic::{Request, Response, Status};

    use crate::rpc::health;
    use crate::rpc::health::{HealthCheckRequest, HealthConfig};

    #[derive(Default)]
    pub struct HealthService;
    #[async_trait]
    impl health::health_server::Health for HealthService {
        async fn check(
            &self,
            _: Request<HealthCheckRequest>,
        ) -> Result<Response<HealthConfig>, Status> {
            let status = HealthConfig::generate();
            Ok(Response::new(status))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::health::client::HealthClients;
    use super::*;
    #[test]
    fn test_health_config() {
        HealthConfig::generate();
    }
    #[tokio::test]
    async fn test_client(){
        let mut client = HealthClients::new("http://127.0.0.1:50051".to_string()).await.unwrap();
        let start = std::time::Instant::now();
        let x = client.check().await.unwrap();
        println!("{:?}",start.elapsed());
        println!("{:?}",x);
    }
}
