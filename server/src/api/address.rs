use crate::api::AppState;
use crate::db;
use crate::models::Address;
use axum::{extract::State, http::StatusCode, Json};
use rand::Rng;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub domain: Option<String>,
}

const LOCAL_PART_LENGTH: usize = 10;

pub async fn generate_address(
    State(state): State<Arc<AppState>>,
    Json(body): Json<Option<GenerateRequest>>,
) -> Result<(StatusCode, Json<Address>), (StatusCode, String)> {
    let domain = match body.and_then(|b| b.domain) {
        Some(ref d) if state.allowed_domains.contains(d) => d.clone(),
        Some(ref d) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Domain '{}' not allowed", d),
            ))
        }
        None => {
            let idx = rand::thread_rng().gen_range(0..state.allowed_domains.len());
            state.allowed_domains[idx].clone()
        }
    };

    let local: String = (0..LOCAL_PART_LENGTH)
        .map(|_| rand::thread_rng().gen_range(b'a'..=b'z') as char)
        .collect();

    let address = Address::new(
        format!("{}@{}", local, domain),
        domain,
        state.email_ttl_minutes,
    );

    db::queries::insert_address(state.db.pool(), &address)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(address)))
}
