pub mod parser;

use crate::db;
use crate::db::Database;
use crate::models::EmailEvent;
use crate::notify;
use crate::notify::NotificationSender;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub const SMTP_PORT: u16 = 25;
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_EMAIL_SIZE: usize = 1_048_576;

#[derive(Debug, Clone, PartialEq, Default)]
enum SmtpState {
    #[default]
    WaitingGreeting,
    WaitingMailFrom,
    WaitingRcptTo,
    WaitingData,
    ReceivingData {
        collected: Vec<u8>,
    },
    MailTransactionComplete,
}

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

    async fn process_line(&mut self, line: &str) -> String {
        let line = line.trim();

        match line.to_uppercase().as_str() {
            "QUIT" => {
                self.state = SmtpState::MailTransactionComplete;
                return "221 Bye\r\n".to_string();
            }
            "RSET" => {
                self.reset_transaction();
                return "250 OK\r\n".to_string();
            }
            "NOOP" => {
                return "250 OK\r\n".to_string();
            }
            _ => {}
        }

        match &self.state {
            SmtpState::WaitingGreeting => self.handle_ehlo(line),
            SmtpState::WaitingMailFrom => self.handle_mail_from(line),
            SmtpState::WaitingRcptTo => self.handle_rcpt_to(line),
            SmtpState::WaitingData => self.handle_data_cmd(line),
            SmtpState::ReceivingData { .. } => self.handle_data_line(line),
            SmtpState::MailTransactionComplete => "500 Transaction ended\r\n".to_string(),
        }
    }

    fn handle_ehlo(&mut self, line: &str) -> String {
        if line.to_uppercase().starts_with("EHLO") || line.to_uppercase().starts_with("HELO") {
            self.state = SmtpState::WaitingMailFrom;
            "250 tmpml.net Hello\r\n".to_string()
        } else {
            "500 Expected EHLO/HELO\r\n".to_string()
        }
    }

    fn handle_mail_from(&mut self, line: &str) -> String {
        if let Some(addr) = Self::extract_email(line, "MAIL FROM:")
            .or_else(|| Self::extract_email(line, "mail from:"))
        {
            self.sender = Some(addr);
            self.state = SmtpState::WaitingRcptTo;
            "250 OK\r\n".to_string()
        } else {
            "501 Malformed MAIL FROM\r\n".to_string()
        }
    }

    fn handle_rcpt_to(&mut self, line: &str) -> String {
        if let Some(addr) =
            Self::extract_email(line, "RCPT TO:").or_else(|| Self::extract_email(line, "rcpt to:"))
        {
            if !self.is_domain_allowed(&addr) {
                return "550 Domain not allowed\r\n".to_string();
            }
            self.recipients.push(addr);
            self.state = SmtpState::WaitingData;
            "250 OK\r\n".to_string()
        } else {
            "501 Malformed RCPT TO\r\n".to_string()
        }
    }

    fn handle_data_cmd(&mut self, _line: &str) -> String {
        if self.sender.is_none() {
            return "503 Need MAIL FROM\r\n".to_string();
        }
        if self.recipients.is_empty() {
            return "503 Need RCPT TO\r\n".to_string();
        }
        self.state = SmtpState::ReceivingData {
            collected: Vec::with_capacity(4096),
        };
        "354 Start mail input; end with <CRLF>.<CRLF>\r\n".to_string()
    }

    fn handle_data_line(&mut self, line: &str) -> String {
        let state = std::mem::take(&mut self.state);
        if let SmtpState::ReceivingData { mut collected } = state {
            if line == "." {
                match self.process_email(collected) {
                    Ok(_) => {
                        self.state = SmtpState::MailTransactionComplete;
                        "250 OK: Message accepted\r\n".to_string()
                    }
                    Err(e) => {
                        self.state = SmtpState::MailTransactionComplete;
                        format!("554 Transaction failed: {}\r\n", e)
                    }
                }
            } else {
                if line.starts_with("..") {
                    collected.extend_from_slice(&line.as_bytes()[1..]);
                } else {
                    collected.extend_from_slice(line.as_bytes());
                }
                collected.extend_from_slice(b"\r\n");

                if collected.len() > MAX_EMAIL_SIZE {
                    self.state = SmtpState::MailTransactionComplete;
                    return "552 Message size exceeds limit\r\n".to_string();
                }
                self.state = SmtpState::ReceivingData { collected };
                String::new()
            }
        } else {
            self.state = SmtpState::ReceivingData {
                collected: Vec::new(),
            };
            "500 Internal error\r\n".to_string()
        }
    }

    fn process_email(&mut self, raw_data: Vec<u8>) -> anyhow::Result<()> {
        let email = parser::parse_email(
            &raw_data,
            self.sender.as_deref().unwrap_or("unknown"),
            &self.recipients,
        )?;

        let pool = self.db.pool();
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { db::queries::insert_email(pool, &email).await })
        })?;

        let event = EmailEvent::from_email(&email);
        notify::send_notification(&self.tx, &event);

        tracing::info!(
            "Email stored: {} -> {} subject='{}'",
            email.from_addr,
            email.to_address,
            email.subject,
        );

        Ok(())
    }

    fn extract_email(line: &str, prefix: &str) -> Option<String> {
        let after = line.strip_prefix(prefix)?.trim();
        let addr = if after.starts_with('<') && after.ends_with('>') {
            &after[1..after.len() - 1]
        } else {
            after
        };
        if addr.contains('@') && !addr.is_empty() {
            Some(addr.to_string())
        } else {
            None
        }
    }

    fn is_domain_allowed(&self, email: &str) -> bool {
        if let Some(at_pos) = email.rfind('@') {
            let domain = &email[at_pos + 1..];
            self.allowed_domains.iter().any(|d| d == domain)
        } else {
            false
        }
    }

    fn reset_transaction(&mut self) {
        self.state = SmtpState::WaitingMailFrom;
        self.sender = None;
        self.recipients.clear();
    }
}

pub async fn start_smtp_server(
    db: Arc<Database>,
    tx: NotificationSender,
    allowed_domains: Vec<String>,
) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", SMTP_PORT);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("SMTP server listening on {}", addr);

    loop {
        let (stream, _peer) = listener.accept().await?;
        let db = db.clone();
        let tx = tx.clone();
        let domains = allowed_domains.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, db, tx, domains).await {
                tracing::error!("SMTP error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    db: Arc<Database>,
    tx: NotificationSender,
    allowed_domains: Vec<String>,
) -> anyhow::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut handler = ConnectionHandler::new(db, tx, allowed_domains);

    writer
        .write_all(b"220 tmpml.net ESMTP TempMail Ready\r\n")
        .await?;

    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = tokio::time::timeout(IDLE_TIMEOUT, reader.read_line(&mut line)).await;

        match bytes_read {
            Ok(Ok(0)) => break,
            Ok(Ok(_)) => {
                let response = handler.process_line(&line).await;
                if !response.is_empty() {
                    writer.write_all(response.as_bytes()).await?;
                    writer.flush().await?;
                }
                if line.trim().to_uppercase() == "QUIT" {
                    break;
                }
            }
            Ok(Err(e)) => {
                tracing::error!("SMTP read error: {:?}", e);
                break;
            }
            Err(_) => {
                let _ = writer.write_all(b"421 Timeout\r\n").await;
                break;
            }
        }
    }
    Ok(())
}
