# syntax=docker/dockerfile:1.6

FROM rust:1.81-slim AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/magic-mail /app/magic-mail
COPY --from=builder /app/migrations /app/migrations

ENV PORT=3001
ENV SMTP_PORT=2525
ENV DATABASE_URL=sqlite:tempmail.db?mode=rwc
EXPOSE 3001

CMD ["/app/magic-mail"]
