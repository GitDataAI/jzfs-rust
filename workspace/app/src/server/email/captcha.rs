use lettre::message::Mailbox;
use config::email::captcha::CAPTCHA;
use crate::server::email::EmailServer;
use crate::server::email::msg::EmailMSG;

impl EmailServer {
    pub async fn send_captcha(&self, email: Mailbox, code: String){
        let tmp = CAPTCHA.to_string();
        let tmp = tmp.replace("123456", &code);
        self.send(EmailMSG{
            from: "gitdata-bot@gitdata.ai".parse().unwrap(),
            reply: "gitdata-bot@gitdata.ai".parse().unwrap(),
            to: email,
            subject: "GitData Captcha".to_string(),
            body: tmp,
        })
    }
}