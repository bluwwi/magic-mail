use crate::models::{Address, Email};
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


