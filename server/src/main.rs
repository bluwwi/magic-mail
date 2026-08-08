mod api;
mod banner;
mod db;
mod models;
mod notify;
mod shutdown;
mod smtp;
mod tasks;

use anyhow::Result;
use db::Database;
use notify::NotificationSender;
use std::sync::Arc;

fn env_or(default: &str, key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_env_u16(key: &str, default: u16) -> u16 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn parse_env_i64(key: &str, default: i64) -> i64 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let database_url = env_or("sqlite:tempmail.db?mode=rwc", "DATABASE_URL");
    db::ensure_db_directory(&database_url)?;
    let db = Arc::new(Database::new(&database_url).await?);

    let tx: NotificationSender = notify::setup_notification_channel();

    let allowed_domains: Vec<String> = std::env::var("ALLOWED_DOMAINS")
        .expect("ALLOWED_DOMAINS env var must be set (comma-separated, e.g. temp.realblue.lol,od3n.online,od3n.info)")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if allowed_domains.is_empty() {
        panic!("ALLOWED_DOMAINS env var is set but empty");
    }

    let http_port = parse_env_u16("PORT", api::HTTP_PORT_DEFAULT);
    let smtp_port = parse_env_u16("SMTP_PORT", smtp::SMTP_PORT_DEFAULT);
    let smtp_hostname = env_or("tmpml.net", "SMTP_HOSTNAME");
    let email_ttl_minutes = parse_env_i64("EMAIL_TTL_MINUTES", 10);

    banner::print_startup_banner(http_port, smtp_port, &allowed_domains);

    let cleanup = tasks::cleanup::CleanupTask::new(db.clone());
    tokio::spawn(async move {
        if let Err(e) = cleanup.run().await {
            tracing::error!("Cleanup task failed: {}", e);
        }
    });

    let db_smtp = db.clone();
    let tx_smtp = tx.clone();
    let domains_smtp = allowed_domains.clone();

    let db_http = db.clone();
    let tx_http = tx.clone();
    let domains_http = allowed_domains;

    let mut smtp_fut = tokio::spawn(async move {
        smtp::start_smtp_server(db_smtp, tx_smtp, domains_smtp, smtp_port, smtp_hostname).await
    });

    let mut http_fut = tokio::spawn(async move {
        api::start_http_server(db_http, tx_http, domains_http, http_port, email_ttl_minutes).await
    });

    tokio::select! {
        _ = shutdown::shutdown_signal() => {
            tracing::info!("Shutdown signal received");
        }
        result = &mut smtp_fut => {
            if let Err(e) = result {
                tracing::error!("SMTP server error: {}", e);
            }
        }
        result = &mut http_fut => {
            if let Err(e) = result {
                tracing::error!("HTTP server error: {}", e);
            }
        }
    }

    tracing::info!("Shutting down...");
    db.close().await;
    tracing::info!("Goodbye!");

    Ok(())
}
