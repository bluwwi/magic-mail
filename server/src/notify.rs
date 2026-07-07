use crate::models::EmailEvent;
use tokio::sync::broadcast;

const CHANNEL_CAPACITY: usize = 128;

pub type NotificationSender = broadcast::Sender<EmailEvent>;
pub type NotificationReceiver = broadcast::Receiver<EmailEvent>;

pub fn setup_notification_channel() -> NotificationSender {
    let (tx, _rx) = broadcast::channel::<EmailEvent>(CHANNEL_CAPACITY);
    tracing::info!(
        "Notification channel created (capacity: {})",
        CHANNEL_CAPACITY
    );
    tx
}

pub fn send_notification(tx: &NotificationSender, event: &EmailEvent) -> bool {
    match tx.send(event.clone()) {
        Ok(receiver_count) => {
            tracing::debug!("Notification sent to {} receiver(s)", receiver_count); 
            receiver_count > 0
        }
        Err(broadcast::error::SendError(_)) => {
            tracing::debug!("No active SSE receivers");
            false
        }
    }
}

pub fn subscribe(tx: &NotificationSender) -> NotificationReceiver {
    tx.subscribe()
}
