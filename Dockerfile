FROM rust:1-bookworm

WORKDIR /app

COPY Cargo.toml ./
COPY src/mail.rs src/mail.rs
RUN echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm src/main.rs target/release/deps/axum_mailexam*

COPY . .
RUN cargo build --release

ENV BIND_ADDR=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["./target/release/axum-mailexam"]
