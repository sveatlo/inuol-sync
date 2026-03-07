use crate::config::EmailConfig;
use anyhow::Result;
use lettre::{
    message::Mailbox,
    transport::smtp::{authentication::Credentials, AsyncSmtpTransport},
    AsyncTransport, Message, Tokio1Executor,
};
use tracing::warn;

#[derive(Debug, Clone)]
pub struct EmailNotifier {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl EmailNotifier {
    pub fn new(config: &EmailConfig) -> Result<Self> {
        let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)?
            .credentials(creds)
            .port(config.smtp_port)
            .build();
        Ok(Self {
            transport,
            from: config.from.parse()?,
            to: config.to.parse()?,
        })
    }

    pub async fn notify_error(&self, error: &anyhow::Error) {
        if let Err(e) = self.try_send(error).await {
            warn!(error = %e, "Failed to send error notification email");
        }
    }

    async fn try_send(&self, error: &anyhow::Error) -> Result<()> {
        let subject = "[inuol-sync] Sync error".to_owned();
        let body = format!("Sync failed\n\nError:\n{error:#}");
        let email = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .subject(subject)
            .body(body)?;
        self.transport.send(email).await?;
        Ok(())
    }
}
