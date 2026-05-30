mod mail;

use axum::{routing::post, Json, Router};
use mail::MailConfig;
use serde::Deserialize;

#[derive(Deserialize)]
struct SendRequest {
    to: String,
    subject: Option<String>,
    body: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = MailConfig::from_env();

    let app = Router::new().route(
        "/mail/test",
        post({
            let config = config.clone();
            move |Json(payload): Json<SendRequest>| {
                let config = config.clone();
                async move {
                    mail::send_test(
                        &config,
                        &payload.to,
                        payload.subject.as_deref().unwrap_or("Axum + Mailexam"),
                        payload.body.as_deref().unwrap_or("Mailexam test from Axum"),
                    )
                    .await
                    .expect("smtp send");
                    "ok"
                }
            }
        }),
    );

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("PORT");

    let listener = tokio::net::TcpListener::bind((bind_addr.as_str(), port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
