use crate::models::Email;
use crate::{api::AppState, db};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct DeleteResponse {
    pub deleted: bool,
}

#[derive(Serialize)]
pub struct ClearResponse {
    pub deleted_count: u64,
}

//list emails
pub async fn list_emails(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Vec<Email>>, (StatusCode, String)> {
    db::queries::get_emails(state.db.pool(), &address)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn get_email(
    State(state): State<Arc<AppState>>,
    Path((_address, id)): Path<(String, String)>,
) -> Result<Json<Email>, (StatusCode, String)> {
    let email = db::queries::get_email(state.db.pool(), &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Email '{}' not found", id)))?;

    let _ = db::queries::mark_email_read(state.db.pool(), &email.id).await;
    Ok(Json(email))
}

pub async fn delete_email(
    State(state): State<Arc<AppState>>,
    Path((_address, id)): Path<(String, String)>,
) -> Result<Json<DeleteResponse>, (StatusCode, String)> {
    let deleted = db::queries::delete_email(state.db.pool(), &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        let _ = db::queries::delete_attachments_for_email(state.db.pool(), &id).await;
        Ok(Json(DeleteResponse { deleted: true }))
    } else {
        Err((StatusCode::NOT_FOUND, format!("Email '{}' not found", id)))
    }
}

pub async fn clear_emails(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<ClearResponse>, (StatusCode, String)> {
    let _ = db::queries::delete_attachments_for_address(state.db.pool(), &address)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let count = db::queries::delete_emails_for_address(state.db.pool(), &address)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ClearResponse {
        deleted_count: count,
    }))
}
