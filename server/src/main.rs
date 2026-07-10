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

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = "sqlite:tempmail.db?mode=rwc";
    let db = Arc::new(Database::new(database_url).await?);
    let tx: NotificationSender = notify::setup_notification_channel();
    let allowed_domains = vec!["realblue.lol".to_string()];

    banner::print_startup_banner(api::HTTP_PORT, smtp::SMTP_PORT, &allowed_domains);

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
        smtp::start_smtp_server(db_smtp, tx_smtp, domains_smtp).await
    });

    let mut http_fut = tokio::spawn(async move {
        api::start_http_server(db_http, tx_http, domains_http).await
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
