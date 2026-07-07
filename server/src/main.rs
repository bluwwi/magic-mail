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
    tracing::info!("MagicMail Backend v{}", env!("CARGO_PKG_VERSION"));

    let database_url = "sqlite::memory:";
    let db = Arc::new(Database::new(database_url).await?);
    let tx: NotificationSender = notify::setup_notification_channel();

    let allowed_domains = vec!["tmpml.net".to_string(), "test.com".to_string()];

    tracing::info!("Allowed domains: {:?}", allowed_domains);
    tracing::info!("SMTP server on port {}", smtp::SMTP_PORT);

    smtp::start_smtp_server(db, tx, allowed_domains).await?;
    Ok(())
}
