use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::Mailbox;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::OnceCell;
use tracing::{error, info};

pub struct EmailMsg{
    pub from: Mailbox,
    pub reply: Mailbox,
    pub to: Mailbox,
    pub subject: String,
    pub body: String,
}
pub static EMAIL:OnceCell<UnboundedSender<EmailMsg>> = OnceCell::const_new();

pub async fn init_email(){
    let (rx, mut tx) = tokio::sync::mpsc::unbounded_channel::<EmailMsg>();
    EMAIL.get_or_init(||async{
        rx
    }).await;
    tokio::spawn(async move {
        let creds = Credentials::new("zy@gitdata.ai".to_owned(), "mwgJyPCiYfT7XRoF".to_owned());
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
}
