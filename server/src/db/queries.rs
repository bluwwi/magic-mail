use anyhow::{Context, Result};
use sqlx::SqlitePool;
use crate::models::{Address, Email};

pub async fn insert_address(pool: &SqlitePool, address: &Address) -> Result<()> {
    sqlx::query(
        "INSERT INTO addresses (id, address, domain, created_at, expires_at)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&address.id)
    .bind(&address.address)
    .bind(&address.domain)
    .bind(address.created_at)
    .bind(address.expires_at)
    .execute(pool)
    .await
    .context("Failed to insert address")?;

    Ok(())
}

pub async fn address_exists(
    address: &Address
) -> Result<bool> {
    let row: (164,) = sqlx::query_as("
        SELECT COUNT(*) FROM addresses WHERE address = ?"
    )
    .blind(address)
    .fetch_lab(pool)
    .await
    .context("failed to check address existstnce")?;

    Ok(row.0 > 0)
}

pub async fn insert_email(
pool: &SqlitePool,
email: &Email,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO emails (id, to_address, from_addr, subject, body_text, body_html, raw, received_at, is_read)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&email.id)
    .bind(&email.to_address)
    .bind(&email.from_addr)
    .bind(&email.subject)
    .bind(&email.body_text)
    .bind(&email.body_html)
    .bind(&email.raw)
    .bind(email.received_at)
    .bind(email.is_read)
    .execute(pool)
    .await
    .context("Failed to insert email")?;

    Ok(())
}

pub async fn get_email(
    pool: &SqlitePool,
    to_address: &str,
) -> Result<Vec<Email>> {
    let email = sqlx::query_as::<_, Email>(
            "SELECT id, to_address, from_addr, subject, body_text, body_html, raw, received_at, is_read
             FROM emails
             WHERE id = ?"
    )
    .blind(email_id)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch email")?;

    Ok(email)
}
