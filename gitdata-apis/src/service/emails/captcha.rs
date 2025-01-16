use std::str::FromStr;
use std::string::ToString;

use actix_session::Session;
use actix_session_ext::SessionExt;
use actix_session_ext::SessionKey;
use apalis::prelude::Storage;
use lettre::Address;
use rand::random;
use serde::Deserialize;
use serde::Serialize;

use crate::jobs::email::EmailJobs;
use crate::jobs::email::EmailType;
use crate::service::AppState;

pub const CAPTCHA_KEY : SessionKey<String> = SessionKey::new("captcha");

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailCaptchaParam {
    pub email : String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EmailCaptchaCheckParam {
    pub email : String,
    pub captcha : String,
}

impl AppState {
    pub async fn email_captcha_send(
        &self,
        param : EmailCaptchaParam,
        session : Session,
    ) -> anyhow::Result<()> {
        let captcha = random::<u32>() % 10000;
        session.insert_by_key(CAPTCHA_KEY, format!("{}:{}", param.email.clone(), captcha))?;
        let email = lettre::message::Mailbox::new(None, Address::from_str(param.email.as_str())?);
        let job = EmailJobs::new(
            "gitdata-bot@gitdata.ai".parse()?,
            "gitdata-bot@gitdata.ai".parse()?,
            email,
            "GitData 验证码".parse()?,
            EmailType::RegistrationVerificationCode,
            captcha.to_string(),
        );
        let mut email_jobs = self
            .email_jobs
            .lock()
            .map_err(|x| anyhow::anyhow!("{}", x.to_string()))?;
        email_jobs
            .push(job)
            .await
            .map_err(|x| anyhow::anyhow!("{}", x.to_string()))?;
        Ok(())
    }
    pub async fn email_captcha_check(
        &self,
        param : EmailCaptchaCheckParam,
        session : Session,
    ) -> anyhow::Result<bool> {
        let captcha_session = session.get_by_key::<String>(CAPTCHA_KEY)?;
        if captcha_session.is_none() {
            return Ok(false);
        }
        let captcha_session = captcha_session.unwrap();
        if captcha_session != format!("{}:{}", param.email, param.captcha) {
            return Ok(false);
        }
        session
            .remove_by_key::<String>(CAPTCHA_KEY)
            .map_err(|x| anyhow::anyhow!("{}", x.to_string()))?;
        Ok(true)
    }
}
