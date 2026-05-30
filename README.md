# Axum + Mailexam

Minimal [Axum](https://github.com/tokio-rs/axum) example that sends test email through [Mailexam](https://mailexam.ru/) SMTP via [lettre](https://crates.io/crates/lettre).

Based on the [Mailexam Axum guide](https://wiki.mailexam.ru/en/examples/axum/).

## What you need

- Rust 1.70+
- A Mailexam account with SMTP credentials for a project

From your Mailexam welcome email or dashboard:

| Variable | Description |
|----------|-------------|
| `MAILEXAM_LOGIN` | SMTP login (host becomes `{login}.mailexam.ru`) |
| `MAILEXAM_PASSWORD` | SMTP password (paired with the login) |
| `MAILEXAM_PORT` | SMTP port (default `587`, STARTTLS) |
| `MAIL_FROM` | Sender address (any test address is fine) |

## Quick start (host)

1. Copy the example environment file and fill in your credentials:

```bash
cp .env.example .env
```

2. Run the server:

```bash
cargo run
```

The server listens on `http://127.0.0.1:8080` by default.

3. Send a test message:

```bash
curl -X POST http://127.0.0.1:8080/mail/test \
  -H 'Content-Type: application/json' \
  -d '{"to":"user@example.test","subject":"Test","body":"Hello"}'
```

On success the endpoint returns `ok`. The message appears in the Mailexam dashboard under your project inbox.

## Environment variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `MAILEXAM_LOGIN` | yes | — | SMTP login; also used to build the host name |
| `MAILEXAM_PASSWORD` | yes | — | SMTP password |
| `MAILEXAM_PORT` | no | `587` | SMTP port (`587`, `2525`, or `465`) |
| `MAIL_FROM` | no | `noreply@example.test` | Sender address |
| `BIND_ADDR` | no | `127.0.0.1` | HTTP bind address |
| `PORT` | no | `8080` | HTTP listen port |

For port **587** the transport uses STARTTLS via `AsyncSmtpTransport::starttls_relay`.

## Project layout

```
.
├── Cargo.toml
├── src/
│   ├── main.rs   # HTTP server and POST /mail/test
│   └── mail.rs   # Mailexam SMTP config and send_test via lettre
├── .env.example
├── Dockerfile         # for local debugging only
└── docker-compose.yml
```

## Docker (debugging)

Docker is provided for local debugging. For day-to-day development, run the app on the host with `cargo run` (see above).

```bash
cp .env.example .env
# edit .env with your credentials

docker compose up --build
```

Then call the same endpoint on the mapped port:

```bash
curl -X POST http://127.0.0.1:8080/mail/test \
  -H 'Content-Type: application/json' \
  -d '{"to":"user@example.test","subject":"Test","body":"Hello"}'
```

Inside the container the server binds to `0.0.0.0:8080` so the port mapping works.

## CI

Set these secrets in your CI environment:

```yaml
variables:
  MAILEXAM_LOGIN: $MAILEXAM_LOGIN
  MAILEXAM_PASSWORD: $MAILEXAM_PASSWORD
  MAILEXAM_PORT: "587"
  MAIL_FROM: "noreply@example.test"
```

After sending a message in a test, verify delivery via the [Mailexam API](https://mailexam.ru/api).

## Troubleshooting

**TLS or connection error**

- Host must be `{login}.mailexam.ru`, where `{login}` matches `MAILEXAM_LOGIN`.
- Login and password must come from the same Mailexam project.
- For port **587** use `starttls_relay`, not SMTPS on 465.

**Message not in the dashboard**

- Open the inbox of the same Mailexam project.
- Check the handler response: SMTP errors are returned from `send_test`.

## See also

- [Mailexam Axum guide (wiki)](https://wiki.mailexam.ru/en/examples/axum/)
- [Actix Web reference implementation](https://github.com/mailexam/Actix) — same mail module, different HTTP framework
- [Mailexam API documentation](https://mailexam.ru/api)
- [Axum documentation](https://docs.rs/axum/latest/axum/)
- [lettre documentation](https://docs.rs/lettre/latest/lettre/)
