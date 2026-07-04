mod api;
mod db;
mod models;
mod notify;
mod smtp;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Magic Backend v{} starting...", env!("CARGO_PKG_VERSION"));

    tracing::info!("SMTP server will bind to port 2525 (local dev)");
    tracing::info!("HTTP server will bind to port 3001");
    tracing::info!("Database: SQLite");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    tracing::info!("Shutting down");
    Ok(())
}
