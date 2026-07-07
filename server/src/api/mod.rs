pub mod address;
pub mod emails;
pub mod sse;

use crate::db::Database;
use crate::notify::NotificationSender;
use axum::Json;
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub struct AppState {
    pub db: Arc<Database>,
    pub tx: NotificationSender,
    pub allowed_domains: Vec<String>,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

pub async fn health_handler(
    State(state): axum::extract::State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let healthy = state.db.health_check().await.unwrap_or(false);
    let uptime = (chrono::Utc::now() - state.started_at).num_seconds();

    Json(serde_json::json!({
        "status": if healthy { "ok" } else { "degraded" },
        "uptime_seconds": uptime,
        "db_connected": healthy,
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
pub async fn list_domains(State(state): axum::extract::State<Arc<AppState>>) -> Json<Vec<String>> {
    Json(state.allowed_domains.clone())
}
