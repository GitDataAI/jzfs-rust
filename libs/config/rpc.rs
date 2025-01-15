use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RpcConfig {
    pub nodes: Vec<RpcNodeType>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Node {
    pub node_id: String,
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum RpcNodeType {
    Health(Node),
    CoreGit(Node),
    GitCore(Node),
}

impl Default for Node {
    fn default() -> Self {
        Self {
            node_id: "node-0".to_string(),
            host: "127.0.0.1".to_string(),
            port: 45453,
        }
    }
}

impl Node {
    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            nodes: vec![
                RpcNodeType::Health(Node::default()),
                RpcNodeType::CoreGit(Node::default()),
                RpcNodeType::GitCore(Node::default()),
            ],
        }
    }
}
impl RpcConfig {
    pub fn save(&self) -> anyhow::Result<()> {
        let config_file = std::env::var("GITDATA_CONFIG_FILE").unwrap_or("./rpc.toml".to_string());
        let config_dir = std::env::var("GITDATA_CONFIG_DIR").unwrap_or("./config".to_string());
        let config_path = std::path::Path::new(&config_dir).join(&config_file);
        let config_dir = config_path.parent().unwrap();
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir)?;
        }
        std::fs::write(config_path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
    pub fn load() -> anyhow::Result<Self> {
        let config_file = std::env::var("GITDATA_CONFIG_FILE").unwrap_or("./rpc.toml".to_string());
        let config_dir = std::env::var("GITDATA_CONFIG_DIR").unwrap_or("./config".to_string());
        let config_path = std::path::Path::new(&config_dir).join(&config_file);
        if !config_path.exists() {
            Self::default().save().ok();
            return Ok(RpcConfig::default());
        }
        let config = std::fs::read_to_string(config_path)?;
        Ok(toml::from_str(&config)?)
    }
    pub fn gitcore_node(&self) -> anyhow::Result<Node> {
        for node in &self.nodes {
            if let RpcNodeType::GitCore(node) = node {
                return Ok(node.clone());
            }
        }
        Err(anyhow::anyhow!("No gitcore node found"))
    }
    pub fn coregit_node(&self) -> anyhow::Result<Node> {
        for node in &self.nodes {
            if let RpcNodeType::CoreGit(node) = node {
                return Ok(node.clone());
            }
        }
        Err(anyhow::anyhow!("No coregit node found"))
    }
    pub fn health_node(&self) -> anyhow::Result<Node> {
        for node in &self.nodes {
            if let RpcNodeType::Health(node) = node {
                return Ok(node.clone());
            }
        }
        Err(anyhow::anyhow!("No health node found"))
    }
}
