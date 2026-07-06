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
