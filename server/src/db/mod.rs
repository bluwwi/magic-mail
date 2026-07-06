pub mod queries;

use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::path::Path;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        tracing::info!("Initializing database: {}", database_url);

        let pool = SqlitePool::connect(database_url)
            .await
            .context("Failed to connect to SQLite database")?;

        sqlx::query("PRAGMA journal_mode=WAL;")
            .execute(&pool)
            .await
            .context("failed to enable WAL mode")?;

        tracing::info!("WAL mode enabled");

        let schema = include_str!("../../migrations/001_init.sql");
        sqlx::query(schema)
            .execute(&pool)
            .await
            .context("Failed to run database migrations")?;

        tracing::info!("Database migrations applied successfully");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn health_check(&self) -> Result<bool> {
        let result: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .context("Database Heaath check failed")?;
        Ok(result.0 == 1)
    }
}

pub fn ensure_db_directory(database_url: &str) -> Result<()> {
    let path_str = database_url
        .strip_prefix("sqlite:")
        .or_else(|| database_url.strip_prefix("sqlite://"))
        .unwrap_or(database_url);

    let path = Path::new(path_str);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }
    }
    Ok(())
}
