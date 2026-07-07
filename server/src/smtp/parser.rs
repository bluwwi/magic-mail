use anyhow::{Context, Result};
use mailparse::{parse_mail, ParsedMail};
use crate::models::Email;

pub fn parse_email(
    raw_data: &[u8],
    envelope_sender: &str,
    envelope_recipients: &[String],
) -> Result<Email> {
    let parsed = parse_mail(raw_data)
        .context("Failed to parse MIME email")?;

    let subject = get_header(&parsed, "Subject").unwrap_or_default();
    let from_addr = get_header(&parsed, "From")
        .unwrap_or_else(|| envelope_sender.to_string());
    let to_address = envelope_recipients.first().cloned()
        .unwrap_or_else(|| get_header(&parsed, "To").unwrap_or_default());

    let (body_text, body_html) = extract_bodies(&parsed);
    let raw_string = String::from_utf8_lossy(raw_data).to_string();

    Ok(Email::new(
        to_address, from_addr, subject,
        body_text, body_html, Some(raw_string),
    ))
}

fn get_header(parsed: &ParsedMail, name: &str) -> Option<String> {
    parsed.headers.iter()
        .find(|h| h.get_key().eq_ignore_ascii_case(name))
        .map(|h| h.get_value())
}

fn extract_bodies(parsed: &ParsedMail) -> (Option<String>, Option<String>) {
    let mut text = None;
    let mut html = None;
    extract_recursive(parsed, &mut text, &mut html);
    (text, html)
}

fn extract_recursive(
    parsed: &ParsedMail,
    text: &mut Option<String>,
    html: &mut Option<String>,
) {
    match parsed.ctype.mimetype.as_str() {
        "text/plain" if text.is_none() => {
            *text = parsed.get_body().ok();
        }
        "text/html" if html.is_none() => {
            *html = parsed.get_body().ok();
        }
        _ => {
            for sub in &parsed.subparts {
                extract_recursive(sub, text, html);
            }
        }
    }
}
