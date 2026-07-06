pub mod parser;

#[derive(Debug, Clone, PartialEq)]
enum SmtpState {
    WaitingGreeting,
    WaitingMailFrom,
    WaitingRcptTo,
    WaitingData,
    ReceivingData { collected: Vec<u8> },
    MailTransactionComplete,
}

use axum::extract::State;

use crate::db::Database;
use crate::models::{Email, EmailEvent};
use crate::notify::NotificationSender;
use std::sync::Arc;

const MAX_EMAIL_SIZE: usize = 1_048_576; // 1 mb

struct ConnectionHandler {
    state: SmtpState,
    sender: Option<String>,
    recipients: Vec<String>,
    db: Arc<Database>,
    tx: NotificationSender,
    allowed_domains: Vec<String>,
}

impl ConnectionHandler {
    fn new(db: Arc<Database>, tx: NotificationSender, allowed_domains: Vec<String>) -> Self {
        Self {
            state: SmtpState::WaitingGreeting,
            sender: None,
            recipients: Vec::new(),
            db,
            tx,
            allowed_domains,
        }
    }
}
