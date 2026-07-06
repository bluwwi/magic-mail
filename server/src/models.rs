use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Address {
    pub id: String,
    pub address: String,
    pub domain: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Email {
    pub id: String,
    pub to_address: String,
    pub from_addr: String,
    pub subject: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub raw: Option<String>,
    pub received_at: i64,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmailEvent {
    pub to_address: String,
    pub email_id: String,
    pub subject: String,
    pub from_addr: String,
}

impl Email {
    pub fn new(
        to_address: String,
        from_addr: String,
        subject: String,
        body_text: Option<String>,
        body_html: Option<String>,
        raw: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            to_address,
            from_addr,
            subject,
            body_text,
            body_html,
            raw,
            received_at: chrono::Utc::now().timestamp(),
            is_read: false,
        }
    }
}

impl Address {
    pub fn new(address: String, domain: String, ttl_minutes: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            address,
            domain,
            created_at: now,
            expires_at: now + (ttl_minutes * 60),
        }
    }
}

impl EmailEvent {
    pub fn from_email(email: &Email) -> Self {
        Self {
            to_address: email.to_address.clone(),
            email_id: email.id.clone(),
            subject: email.subject.clone(),
            from_addr: email.from_addr.clone(),
        }
    }
}
