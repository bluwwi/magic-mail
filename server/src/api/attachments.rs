use crate::api::AppState;
use crate::db;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
};
use std::sync::Arc;

pub async fn get_attachment(
    State(state): State<Arc<AppState>>,
    Path((email_id, cid)): Path<(String, String)>,
) -> Result<Response, (StatusCode, String)> {
    let att = db::queries::get_attachment_by_cid(state.db.pool(), &email_id, &cid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Attachment not found".to_string()))?;

    let mut resp = Response::new(Body::from(att.data));
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        att
            .content_type
            .parse()
            .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
    );
    headers.insert(header::CACHE_CONTROL, "private, max-age=3600".parse().unwrap());
    Ok(resp)
}
