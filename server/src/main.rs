mod api;
mod db;
mod models;
mod notify;
mod smtp;

use anyhow::Result;
use db::Database;
use models::{Address, Email};
use notify::NotificationSender;
use std::sync::Arc;
use tracing::event;

use crate::models::EmailEvent;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Magic Backend v{} starting...", env!("CARGO_PKG_VERSION"));

    let database_url = "sqlite::memory:";
    let db = Arc::new(Database::new(database_url).await?);
    let tx: NotificationSender = notify::setup_notification_channel();

    let address = Address::new("test@tmpml.net".to_string(), "tmpml.net".to_string(), 10);
    db::queries::insert_address(db.pool(), &address).await?;

    let email = Email::new(
        "test@tmpml.net".to_string(),
        "alice@example.com".to_string(),
        "Hello from blue!".to_string(),
        Some("Plain text body".to_string()),
        Some("<h1>Hello</h1><p>HTML body</p>".to_string()),
        None,
    );
    db::queries::insert_email(db.pool(), &email).await?;

    //test notification
    let event = EmailEvent::from_email(&email);
    notify::send_notification(&tx, &event);
    tracing::info!("Sent notification for: {}", event.subject);

    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();

    let event2 = EmailEvent {
        to_address: "test@tmpml.net".to_string(),
        email_id: "second-email".to_string(),
        subject: "Second email".to_string(),
        from_addr: "bob@example.com".to_string(),
    };

    tx.send(event2.clone())?;

    let from_rx2 = rx2.recv().await?;
    let from_rx3 = rx3.recv().await?;
    assert_eq!(from_rx2, event2);
    assert_eq!(from_rx3, event2);
    tracing::info!("Multiple subscriber test passed!");

    tracing::info!("");
    tracing::info!("=== Complete ===");
    tracing::info!("Notifications: broadcast channel — working");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    tracing::info!("Shutting down.");
    Ok(())
}
