use std::io;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::OnceCell;
use tracing::{error, info};
use gitdata::config::email::EmailConfig;
use crate::public::CAPTCHA;
use crate::public::USER_FOR_GET_PASSWD;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailJobs {
    pub from : Mailbox,
    pub reply : Mailbox,
    pub to : Mailbox,
    pub subject : String,
    pub body : EmailType,
    pub code: String,
}

impl EmailJobs {
    pub fn new(
        from : Mailbox,
        reply : Mailbox,
        to : Mailbox,
        subject : String,
        body : EmailType,
        code: String,
    ) -> Self {
        EmailJobs {
            from,
            reply,
            to,
            subject,
            body,
            code,
        }
    }
}

pub async fn send_email(tx : EmailJobs) -> Result<(), io::Error> {
    let email_server = EmailServer::get().await;
    let email = Message::builder()
        .from(tx.from)
        .reply_to(tx.reply)
        .to(tx.to.clone())
        .subject(tx.subject)
        .header(ContentType::TEXT_HTML)
        .body(tx.body.to_email().replace("123456", &tx.code))
        .unwrap();
    match email_server.cred.send(email).await {
        Ok(_) => info!("Email sent {} successfully!", tx.to.to_string()),
        Err(e) => error!("Could not send email: {e:?}"),
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum EmailType {
    RegistrationVerificationCode,
    ResetPasswordVerificationCode,
}

impl EmailType {
    pub fn to_email(&self) -> String {
        match self {
            EmailType::RegistrationVerificationCode => CAPTCHA.to_string(),
            EmailType::ResetPasswordVerificationCode => USER_FOR_GET_PASSWD.to_string(),
        }
    }
}

static EMAIL: OnceCell<EmailServer> = OnceCell::const_new();

#[derive(Clone)]
struct EmailServer{
    cred: AsyncSmtpTransport<Tokio1Executor>
}

impl EmailServer {
    pub fn init() -> EmailServer {
        let config = EmailConfig::load().expect("Failed to load email config");
        let creds =
            Credentials::new(config.username.to_owned(), config.password.to_owned());
        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp)
                .unwrap()
                .credentials(creds)
                .build();
        EmailServer{
            cred: mailer
        }
    }
    pub async fn get() -> Self{
        EMAIL.get_or_init(|| async {
            EmailServer::init()
        }).await.clone()
    }
}