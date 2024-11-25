use jzfs_entity::config::anyhow;
use crate::Module;

impl Module {
    pub async fn email_send_captcha(&self, email: String, code: String) -> anyhow::Result<()>{
        self.email.send_captcha(email.parse()?, code).await;
        Ok(())
    }
}