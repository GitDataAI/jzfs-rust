use deadpool_redis::redis::AsyncCommands;
use lettre::message::Mailbox;
use uuid::Uuid;
use config::redis::REDIS;
use crate::service::email::EmailService;

impl EmailService {
    pub async fn send_forget_token(&self, email: String) -> anyhow::Result<()>{
        let uuid = sha256::digest(Uuid::new_v4().to_string());
        let mut redis = REDIS.get().unwrap().write()
            .map_err(|x|{
                log::error!("[Error] {}", x);
                anyhow::anyhow!("[Error] {}", x)
            })?;
        redis.set::<String,String,String>(uuid.clone(), email.clone()).await.ok();
        redis.expire::<String,String>(uuid.clone(),3600).await.ok();
        let mail = email.parse::<Mailbox>()
            .map_err(|e|{
                log::error!("[Error] {}", e);
                anyhow::anyhow!("[Error] {}", e)
            })?;
        self.email.send_forget_token(mail, uuid).await;
        Ok(())
    }
}