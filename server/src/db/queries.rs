use crate::models::{Address, Email, Attachment};
use anyhow::{Context, Result};
use sqlx::SqlitePool;

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

pub async fn insert_email(pool: &SqlitePool, email: &Email) -> Result<()> {
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

pub async fn get_email(pool: &SqlitePool, email_id: &str) -> Result<Option<Email>> {
    let email = sqlx::query_as::<_, Email>(
        "SELECT id, to_address, from_addr, subject, body_text, body_html, raw, received_at, is_read
         FROM emails
         WHERE id = ?",
    )
    .bind(email_id)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch email")?;

    Ok(email)
}

pub async fn get_emails(pool: &SqlitePool, to_address: &str) -> Result<Vec<Email>> {
    let emails = sqlx::query_as::<_, Email>(
        "SELECT id, to_address, from_addr, subject, body_text, body_html, raw, received_at, is_read
         FROM emails
         WHERE to_address = ?
         ORDER BY received_at DESC",
    )
    .bind(to_address)
    .fetch_all(pool)
    .await
    .context("Failed to fetch emails")?;

    Ok(emails)
}

pub async fn delete_email(pool: &SqlitePool, email_id: &str) -> Result<bool> {
    let result = sqlx::query("DELETE FROM emails WHERE id = ?")
        .bind(email_id)
        .execute(pool)
        .await
        .context("Failed to delete email")?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_emails_for_address(pool: &SqlitePool, to_address: &str) -> Result<u64> {
    let result = sqlx::query("DELETE FROM emails WHERE to_address = ?")
        .bind(to_address)
        .execute(pool)
        .await
        .context("Failed to delete emails for address")?;

    Ok(result.rows_affected())
}

pub async fn mark_email_read(pool: &SqlitePool, email_id: &str) -> Result<()> {
    sqlx::query("UPDATE emails SET is_read = 1 WHERE id = ?")
        .bind(email_id)
        .execute(pool)
        .await
        .context("Failed to mark email as read")?;

    Ok(())
}

pub async fn cleanup_expired(pool: &SqlitePool, batch_size: u64) -> Result<u64> {
    let batch = batch_size as i64;
    let now = chrono::Utc::now().timestamp();

    let orphaned_attachments = sqlx::query(
        "DELETE FROM attachments WHERE email_id IN (
            SELECT e.id FROM emails e
            LEFT JOIN addresses a ON e.to_address = a.address
            WHERE a.id IS NULL OR a.expires_at < ?
            LIMIT ?
        )",
    )
    .bind(now)
    .bind(batch)
    .execute(pool)
    .await
    .context("Failed to clean up orphaned attachments")?
    .rows_affected();

    let deleted_emails = sqlx::query(
        "DELETE FROM emails WHERE id IN (
            SELECT e.id FROM emails e
            LEFT JOIN addresses a ON e.to_address = a.address
            WHERE a.id IS NULL OR a.expires_at < ?
            LIMIT ?
        )",
    )
    .bind(now)
    .bind(batch)
    .execute(pool)
    .await
    .context("Failed to clean up expired emails")?
    .rows_affected();

    let deleted_addresses = sqlx::query("DELETE FROM addresses WHERE expires_at < ?")
        .bind(now)
        .execute(pool)
        .await
        .context("Failed to clean up expired addresses")?
        .rows_affected();

    Ok(orphaned_attachments + deleted_emails + deleted_addresses)
}

pub async fn insert_attachment(pool: &SqlitePool, email_id: &str, att: &Attachment) -> Result<()> {
    sqlx::query(
        "INSERT INTO attachments (id, email_id, cid, content_type, filename, data, inline)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&att.id)
    .bind(email_id)
    .bind(&att.cid)
    .bind(&att.content_type)
    .bind(&att.filename)
    .bind(&att.data)
    .bind(att.inline)
    .execute(pool)
    .await
    .context("Failed to insert attachment")?;
    Ok(())
}

pub async fn get_attachment_by_cid(
    pool: &SqlitePool,
    email_id: &str,
    cid: &str,
) -> Result<Option<Attachment>> {
    let row = sqlx::query_as::<_, Attachment>(
        "SELECT id, email_id, cid, content_type, filename, data, inline
         FROM attachments
         WHERE email_id = ? AND cid = ?",
    )
    .bind(email_id)
    .bind(cid)
    .fetch_optional(pool)
    .await
    .context("Failed to fetch attachment")?;
    Ok(row)
}

pub async fn delete_attachments_for_email(pool: &SqlitePool, email_id: &str) -> Result<u64> {
    let result = sqlx::query("DELETE FROM attachments WHERE email_id = ?")
        .bind(email_id)
        .execute(pool)
        .await
        .context("Failed to delete attachments")?;
    Ok(result.rows_affected())
}

pub async fn delete_attachments_for_address(pool: &SqlitePool, to_address: &str) -> Result<u64> {
    let result = sqlx::query(
        "DELETE FROM attachments WHERE email_id IN (
            SELECT id FROM emails WHERE to_address = ?
        )",
    )
    .bind(to_address)
    .execute(pool)
    .await
    .context("Failed to delete attachments for address")?;
    Ok(result.rows_affected())
}
