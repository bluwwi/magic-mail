pub mod address;
pub mod emails;
pub mod sse;

use crate::db::Database;
use crate::notify::NotificationSender;
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<Database>,
    pub tx: NotificationSender,
    pub allowed_domains: Vec<String>,
    pub started_at: chrono::DateTime<chrono::Utc>,
}
