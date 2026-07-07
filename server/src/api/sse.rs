use crate::api::AppState;
use crate::models::EmailEvent;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

pub async fn sse_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(address): axum::extract::Path<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let existing = db::queries::get_emails(state.db.pool(), &address)
        .await
        .unwrap_or_default();

    let initial = existing.into_iter().map(|email| {
        let event = EmailEvent::from_email(&email);
        let data = serde_json::to_string(&event).unwrap_or_default();
        Ok::<_, Infallible>(Event::default().event("new_email").data(data))
    });

    let rx = state.tx.subscribe();
    let live = BroadcastStream::new(rx).filter_map(move |result| {
        let addr = address.clone();
        async move {
            match result {
                Ok(event) if event.to_address == addr => {
                    let data = serde_json::to_string(&event).unwrap_or_default();
                    Some(Ok::<_, Infallible>(
                        Event::default().event("new_email").data(data),
                    ))
                }
                _ => None,
            }
        }
    });

    let stream = futures::stream::iter(initial).chain(live);

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text(":ping"),
    )
}
