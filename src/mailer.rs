use lettre::message::header::ContentType;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;

use crate::config::SmtpConfig;

pub struct Mailer;

#[derive(Debug, Clone)]
pub struct Mail {
    pub to: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug)]
pub enum MailError {
    Transport(lettre::transport::smtp::Error),
    Address(lettre::address::AddressError),
    Mail(lettre::error::Error),
}

impl From<lettre::transport::smtp::Error> for MailError {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        Self::Transport(value)
    }
}

impl From<lettre::address::AddressError> for MailError {
    fn from(value: lettre::address::AddressError) -> Self {
        Self::Address(value)
    }
}

impl From<lettre::error::Error> for MailError {
    fn from(value: lettre::error::Error) -> Self {
        Self::Mail(value)
    }
}

impl std::fmt::Display for MailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MailError::Transport(error) => error.fmt(f),
            MailError::Address(error) => error.fmt(f),
            MailError::Mail(error) => error.fmt(f),
        }
    }
}

impl Mailer {
    pub async fn send(config: SmtpConfig, mail: Mail) -> Result<(), MailError> {
        let SmtpConfig { from, host, port, username, password } = config;
        let Mail { subject, to, body } = mail;

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)?
            .credentials(Credentials::new(username, password))
            .port(port)
            .build();

        let email = Message::builder()
            .from(from.parse::<Mailbox>()?)
            .to(to.parse::<Mailbox>()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        AsyncTransport::send(&mailer, email).await?;

        Ok(())
    }
}