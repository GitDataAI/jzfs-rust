use log::warn;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

#[derive(Deserialize,Serialize,Clone,Debug, Default)]
pub struct GitConfig {
    pub http: String,
    pub ssh: String,
}

static GIT_CONFIG: OnceCell<GitConfig> = OnceCell::const_new();


impl GitConfig {
    pub fn get() -> Self {
        if let Some(config) = GIT_CONFIG.get() {
            return config.clone();
        }
        let config = GitConfig::load().unwrap_or_default();
        GIT_CONFIG.set(config.clone()).unwrap();
        config
    }
    pub fn new(http: String, ssh: String) -> Self {
        GitConfig { http, ssh }
    }
    pub fn save(&self) -> anyhow::Result<()> {
        if std::fs::read_dir("./config").is_err(){
            std::fs::create_dir("./config")?;
        }
        std::fs::write("./config/git.toml", toml::to_string(self)?)?;
        Ok(())
    }
    pub fn load() -> anyhow::Result<Self> {
        if std::fs::read_dir("./config").is_err(){
            std::fs::create_dir("./config")?;
            Self::default().save()?;
            warn!("git config not found, use default config.");
            return Ok(Self::default());
        }
        let config = match std::fs::read_to_string("./config/git.toml"){
            Ok(config) => config,
            Err(_) => {
                Self::default().save()?;
                warn!("git config not found, use default config.");
                return Ok(Self::default());
            }
        };
        let config = match toml::from_str::<GitConfig>(&config){
            Ok(config) => config,
            Err(_) => {
                Self::default().save()?;
                warn!("git config not found, use default config.");
                return Ok(Self::default());
            }
        };
        Ok(config)
    }
    
}