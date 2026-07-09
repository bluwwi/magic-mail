use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct CleanupTask {
    db: Arc<crate::db::Database>,
    interval: Duration,
    batch_size: u64,
}

impl CleanupTask {
    pub fn new(db: Arc<crate::db::Database>) -> Self {
        Self {
            db,
            interval: Duration::from_secs(60),
            batch_size: 100,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        tracing::info!(
            "Cleanup task started (interval: {:?}, batch: {})",
            self.interval,
            self.batch_size
        );

        loop {
            sleep(self.interval).await;

            let deleted = match crate::db::queries::cleanup_expired(
                self.db.pool(),
                self.batch_size,
            )
            .await
            {
                Ok(count) => count,
                Err(e) => {
                    tracing::warn!("Cleanup query failed: {}", e);
                    continue;
                }
            };

            if deleted > 0 {
                tracing::info!("Cleaned up {} expired record(s)", deleted);
            }
        }
    }
}
