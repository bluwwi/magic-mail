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

pub async fn delete_email(
    pool: &SqlitePool,
    email_id: &str,
) -> Result<bool> {
    let result = sqlx::query("DELETE FROM emails WHERE id = ?")
        .bind(email_id)
        .execute(pool)
        .await
        .context("Failed to delete email")?;

    Ok(result.rows_affected() > 0)
}


pub async fn delete_emails_for_address(
    pool: &SqlitePool,
    to_address: &str,
) -> Result<u64> {
    let result = sqlx::query("DELETE FROM emails WHERE to_address = ?")
        .bind(to_address)
        .execute(pool)
        .await
        .context("Failed to delete emails for address")?;

    Ok(result.rows_affected())
}

pub async fn mark_email_read(
    pool: &SqlitePool,
    email_id: &str,
) -> Result<()> {
    sqlx::query("UPDATE emails SET is_read = 1 WHERE id = ?")
        .bind(email_id)
        .execute(pool)
        .await
        .context("Failed to mark email as read")?;

    Ok(())
}

pub async fn delete_expired_emails(
    pool: &SqlitePool,
    before: i64,
) -> Result<u64> {
    let result = sqlx::query("DELETE FROM emails WHERE received_at < ?")
        .bind(before)
        .execute(pool)
        .await
        .context("Failed to delete expired emails")?;

    Ok(result.rows_affected())
}

pub async fn delete_expired_addresses(
    pool: &SqlitePool,
    now: i64,
) -> Result<u64> {
    let result = sqlx::query("DELETE FROM addresses WHERE expires_at < ?")
        .bind(now)
        .execute(pool)
        .await
        .context("Failed to delete expired addresses")?;

    Ok(result.rows_affected())
}


pub async fn get_email_count(
    pool: &SqlitePool,
    to_address: &str,
) -> Result<i64> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM emails WHERE to_address = ?"
    )
    .bind(to_address)
    .fetch_one(pool)
    .await
    .context("Failed to count emails")?;

    Ok(count)
}

pub async fn checkpoint_wal(pool: &SqlitePool) -> Result<()> {
    sqlx::query("PRAGMA wal_checkpoint(TRUNCATE);")
        .execute(pool)
        .await
        .context("Failed to checkpoint WAL")?;

    tracing::debug!("WAL checkpoint completed");
    Ok(())
}
✅ Commit:
