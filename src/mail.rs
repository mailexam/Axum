use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[derive(Clone)]
pub struct MailConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
}

impl MailConfig {
    pub fn from_env() -> Self {
        let login = std::env::var("MAILEXAM_LOGIN").expect("MAILEXAM_LOGIN");
        Self {
            host: format!("{login}.mailexam.ru"),
            port: std::env::var("MAILEXAM_PORT")
                .unwrap_or_else(|_| "587".into())
                .parse()
                .expect("MAILEXAM_PORT"),
            username: login,
            password: std::env::var("MAILEXAM_PASSWORD").expect("MAILEXAM_PASSWORD"),
            from: std::env::var("MAIL_FROM")
                .unwrap_or_else(|_| "noreply@example.test".into()),
        }
    }
}

pub async fn send_test(
    config: &MailConfig,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from(config.from.parse().expect("MAIL_FROM"))
        .to(to.parse().expect("recipient"))
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .expect("message");

    let creds = Credentials::new(config.username.clone(), config.password.clone());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)?
        .port(config.port)
        .credentials(creds)
        .build();

    mailer.send(email).await.map(|_| ())
}
