pub mod address;
pub mod emails;
pub mod sse;

use crate::db::Database;
use crate::notify::NotificationSender;
use axum::extract::State;
use axum::Json;
use axum::Router;
use std::sync::Arc;
use tower_http::cors;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub const HTTP_PORT: u16 = 3001;

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

pub fn create_router(
    db: Arc<crate::db::Database>,
    tx: crate::notify::NotificationSender,
    allowed_domains: Vec<String>,
) -> Router {
    let state = Arc::new(AppState {
        db,
        tx,
        allowed_domains,
        started_at: chrono::Utc::now(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
        ])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    Router::new()
        .route("/api/health", axum::routing::get(health_handler))
        .route("/api/domains", axum::routing::get(list_domains))
        .route(
            "/api/address/generate",
            axum::routing::post(address::generate_address),
        )
        .route(
            "/api/emails/:address",
            axum::routing::get(emails::list_emails).delete(emails::clear_emails),
        )
        .route(
            "/api/emails/:address/:id",
            axum::routing::get(emails::get_email).delete(emails::delete_email),
        )
        .route("/sse/inbox/:address", axum::routing::get(sse::sse_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

pub async fn start_http_server(
    db: Arc<crate::db::Database>,
    tx: crate::notify::NotificationSender,
    allowed_domains: Vec<String>,
) -> anyhow::Result<()> {
    let app = create_router(db, tx, allowed_domains);
    let addr = format!("0.0.0.0:{}", HTTP_PORT);
    tracing::info!("HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
