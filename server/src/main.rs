mod api;
mod db;
mod models;
mod notify;
mod smtp;

use anyhow::Result;
use db::Database;
use notify::NotificationSender;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("TempMail Backend v{} — Phase 5", env!("CARGO_PKG_VERSION"));

    let database_url = "sqlite::memory:";
    let db = Arc::new(Database::new(database_url).await?);
    let tx: NotificationSender = notify::setup_notification_channel();
    let allowed_domains = vec!["tmpml.net".to_string(), "test.com".to_string()];

    tracing::info!("SMTP server on port {}...", smtp::SMTP_PORT);
    tracing::info!("HTTP server on port {}...", api::HTTP_PORT);

    let db_smtp = db.clone();
    let tx_smtp = tx.clone();
    let domains_smtp = allowed_domains.clone();

    let db_http = db.clone();
    let tx_http = tx.clone();
    let domains_http = allowed_domains.clone();

    let smtp = tokio::spawn(async move { smtp::start_smtp_server(db_smtp, tx_smtp, domains_smtp).await });
    let http = tokio::spawn(async move { api::start_http_server(db_http, tx_http, domains_http).await });

    smtp.await.unwrap()?;
    http.await.unwrap()?;

    Ok(())
}
