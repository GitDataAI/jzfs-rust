use std::io;

use lettre::message::Mailbox;
use serde::Deserialize;
use serde::Serialize;

use crate::public::CAPTCHA;
use crate::public::USER_FOR_GET_PASSWD;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EmailJobs {
    pub from : Mailbox,
    pub reply : Mailbox,
    pub to : Mailbox,
    pub subject : String,
    pub body : EmailType,
}

impl EmailJobs {
    pub fn new(
        from : Mailbox,
        reply : Mailbox,
        to : Mailbox,
        subject : String,
        body : EmailType,
    ) -> Self {
        EmailJobs {
            from,
            reply,
            to,
            subject,
            body,
        }
    }
}

pub async fn send_email(job : EmailJobs) -> Result<(), io::Error> {
    dbg!(job);
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
