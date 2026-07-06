mod api;
mod db;
mod models;
mod notify;
mod smtp;

use anyhow::Result;
use db::Database;
use models::{Address, Email};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Magic Backend v{} starting...", env!("CARGO_PKG_VERSION"));

    let database_url = "sqlite::memory:";
    db::ensure_db_directory(database_url)?;
    let db = Database::new(database_url).await?;

    let healthy = db.health_check().await?;
    tracing::info!("Database health check: {}", healthy);

    let pool = db.pool();

    let address = Address::new("test@tmpml.net".to_string(), "tmpml.net".to_string(), 10);
    db::queries::insert_address(pool, &address).await?;
    tracing::info!("Inserted address: {}", address.address);

    let email = Email::new(
        "test@tmpml.net".to_string(),
        "sender@example.com".to_string(),
        "Welcome to TempMail!".to_string(),
        Some("This is a test email.".to_string()),
        Some("<h1>Test</h1><p>This is HTML</p>".to_string()),
        Some("raw email data".to_string()),
    );
    db::queries::insert_email(pool, &email).await?;
    tracing::info!("Inserted email: {}", email.subject);

    let emails = db::queries::get_emails(pool, "test@tmpml.net").await?;
    tracing::info!("Found {} email(s)", emails.len());

    db::queries::mark_email_read(pool, &email.id).await?;
    let found = db::queries::get_email(pool, &email.id).await?.unwrap();
    tracing::info!("Email read status: {}", found.is_read);

    let deleted = db::queries::delete_expired_emails(pool, 0).await?;
    tracing::info!("Deleted {} expired email(s)", deleted);

    tracing::info!("");
    tracing::info!("=== Phase 2 Complete ===");
    tracing::info!("Database layer is fully functional");

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    tracing::info!("Shutting down. Phase 2 complete!");
    Ok(())
}
