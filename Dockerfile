FROM rust:1.56.1 as builder
WORKDIR /usr/src/telegram_notifier
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/* ./src/
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates tzdata && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/telegram_notifier /usr/local/bin/telegram_notifier
CMD ["telegram_notifier"]