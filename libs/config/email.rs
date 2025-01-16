use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct EmailConfig {
    #[serde(default)]
    pub smtp: String,
    #[serde(default)]
    pub port: u32,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub from: String,
}

impl EmailConfig {
    pub fn new(
        smtp: String,
        port: u32,
        username: String,
        password: String,
        from: String,
    ) -> Self {
        EmailConfig {
            smtp,
            port,
            username,
            password,
            from,
        }
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let config_file = std::env::var("CONFIG_FILE").unwrap_or("./config/email.toml".to_string());
        std::fs::write(config_file, toml::to_string_pretty(self)?)?;
        Ok(())
    }
    pub fn load() -> anyhow::Result<Self> {
        let config_file = std::env::var("CONFIG_FILE").unwrap_or("./config/email.toml".to_string());
        let config = std::fs::read_to_string(config_file)?;
        Ok(toml::from_str(&config)?)
    }
    pub fn check(&self) -> anyhow::Result<()> {
        if self.smtp.is_empty() {
            Self::default().save()?;
            return Err(anyhow::anyhow!("smtp is empty"));
        }
        if self.port == 0 {
            Self::default().save()?;
            return Err(anyhow::anyhow!("port is empty"));
        }
        if self.username.is_empty() {
            Self::default().save()?;
            return Err(anyhow::anyhow!("username is empty"));
        }
        if self.password.is_empty() {
            Self::default().save()?;
            return Err(anyhow::anyhow!("password is empty"));
        }
        if self.from.is_empty() {
            Self::default().save()?;
            return Err(anyhow::anyhow!("from is empty"));
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_config() {
        EmailConfig::default().save().ok();
    }
}