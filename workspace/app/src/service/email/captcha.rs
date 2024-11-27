use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::service::email::EmailService;

impl EmailService {
    pub async fn generate_and_send_captcha(&self, email: String) -> anyhow::Result<String>{
        let rng = rand::thread_rng();
        let code: String = rng.sample_iter(&Alphanumeric).take(6).map(char::from).collect();
        self.email.send_captcha(email.parse().map_err(|e|{
            log::error!("[Error] {}", e);
            anyhow::anyhow!("[Error] {}", e)
        })?, code.clone()).await;
        Ok(code)
    }
}