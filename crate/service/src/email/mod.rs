use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use jzfs_config::tokio;
use jzfs_config::tokio::sync::mpsc::UnboundedSender;
use jzfs_config::tracing::{error, info};
use crate::email::msg::EmailMSG;

pub mod msg;
pub mod captcha;

#[derive(Clone)]
pub struct EmailService{
    rx: UnboundedSender<EmailMSG>
}


impl EmailService {
    pub fn init() -> EmailService {
        let (rx, mut tx) = tokio::sync::mpsc::unbounded_channel::<EmailMSG>();
        tokio::spawn(async move {
            let creds = Credentials::new("gitdata-bot@gitdata.ai".to_owned(), "GsMKT8AP5xf6RUGq".to_owned());
            let mailer: AsyncSmtpTransport<Tokio1Executor> =
                AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.exmail.qq.com")
                    .unwrap()
                    .credentials(creds)
                    .build();
            while let Some(tx) = tx.recv().await {
                let email = Message::builder()
                    .from(tx.from)
                    .reply_to(tx.reply)
                    .to(tx.to.clone())
                    .subject(tx.subject)
                    .header(ContentType::TEXT_HTML)
                    .body(tx.body)
                    .unwrap();
                match mailer.send(email).await {
                    Ok(_) => info!("Email sent {} successfully!", tx.to.to_string()),
                    Err(e) => error!("Could not send email: {e:?}"),
                }
            }
        });
        Self{
            rx
        }
    }
    pub fn send(&self, msg: EmailMSG){
        self.rx.send(msg).unwrap();
    }
    
}