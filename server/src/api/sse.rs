use crate::api::AppState;
use crate::db;
use crate::models::EmailEvent;
use axum::extract::{Path, Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::Stream;
use futures::StreamExt;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

#[derive(Deserialize, Debug)]
pub struct SseQuery {
    pub last_event_id: Option<i64>,
}

fn normalize_address(addr: &str) -> String {
    addr.trim().to_lowercase()
}

fn start_polling(
    tx: &broadcast::Sender<EmailEvent>,
    address: &str,
) -> impl Stream<Item = Result<Event, Infallible>> {
    let rx = tx.subscribe();
    let addr = address.to_string();
    BroadcastStream::new(rx).filter_map(move |result| {
        let addr = addr.clone();
        async move {
            match result {
                Ok(event) if event.to_address == addr => {
                    let data = serde_json::to_string(&event).unwrap_or_default();
                    Some(Ok::<_, Infallible>(
                        Event::default()
                            .event("new_email")
                            .data(data)
                            .id(event.email_id.clone()),
                    ))
                }
                _ => None,
            }
        }
    })
}
pub async fn sse_handler(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<SseQuery>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let address = normalize_address(&address);

    let pool = state.db.pool().clone();

    let existing = db::queries::get_emails(&pool, &address)
        .await
        .unwrap_or_default();

    let initial = existing
        .into_iter()
        .filter(move |e| {
            query
                .last_event_id
                .map(|id| e.received_at > id)
                .unwrap_or(true)
        })
        .map(|email| {
            let event = EmailEvent::from_email(&email);
            let data = serde_json::to_string(&event).unwrap_or_default();
            Ok::<_, Infallible>(
                Event::default()
                    .event("new_email")
                    .data(data)
                    .id(email.id.clone()),
            )
        });

    let live = start_polling(&state.tx, &address);

    let stream = futures::stream::iter(initial).chain(live);
    let addr_for_log = address.clone();
    let stream = stream.inspect(move |_result| {});

    tracing::debug!("SSE connected: {}", addr_for_log);

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text(":ping"),
    )
}
