use lettre::message::Mailbox;
use jzfs_config::email::CAPTCHA_TMP;
use crate::email::EmailService;
use crate::email::msg::EmailMSG;

impl EmailService {
    pub async fn send_captcha(&self, email: Mailbox, code: String){
        let tmp = CAPTCHA_TMP.to_string();
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