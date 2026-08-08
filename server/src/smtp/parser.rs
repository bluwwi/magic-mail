use anyhow::{Context, Result};
use mailparse::{parse_mail, ParsedMail};
use crate::models::{Email, Attachment};

pub struct ParsedEmail {
    pub email: Email,
    pub attachments: Vec<Attachment>,
}

pub fn parse_email(
    raw_data: &[u8],
    envelope_sender: &str,
    envelope_recipients: &[String],
) -> Result<ParsedEmail> {
    let parsed = parse_mail(raw_data).context("Failed to parse MIME email")?;

    let subject = get_header(&parsed, "Subject").unwrap_or_default();
    let from_addr = get_header(&parsed, "From")
        .unwrap_or_else(|| envelope_sender.to_string());
    let to_address = envelope_recipients.first().cloned()
        .unwrap_or_else(|| get_header(&parsed, "To").unwrap_or_default());

    let (body_text, body_html, attachments) = extract_bodies(&parsed);
    let raw_string = String::from_utf8_lossy(raw_data).to_string();

    let mut email = Email::new(
        to_address, from_addr, subject,
        body_text, body_html, Some(raw_string),
    );

    rewrite_cid_links(&mut email, &attachments);

    Ok(ParsedEmail { email, attachments })
}

fn get_header(parsed: &ParsedMail, name: &str) -> Option<String> {
    parsed.headers.iter()
        .find(|h| h.get_key().eq_ignore_ascii_case(name))
        .map(|h| h.get_value())
}

fn extract_bodies(parsed: &ParsedMail) -> (Option<String>, Option<String>, Vec<Attachment>) {
    let mut text = None;
    let mut html = None;
    let mut attachments: Vec<Attachment> = Vec::new();
    extract_recursive(parsed, &mut text, &mut html, &mut attachments);
    (text, html, attachments)
}

fn extract_recursive(
    parsed: &ParsedMail,
    text: &mut Option<String>,
    html: &mut Option<String>,
    attachments: &mut Vec<Attachment>,
) {
    let mime = parsed.ctype.mimetype.as_str().to_lowercase();

    match mime.as_str() {
        "text/plain" if text.is_none() => {
            *text = parsed.get_body().ok();
            return;
        }
        "text/html" if html.is_none() => {
            *html = parsed.get_body().ok();
            return;
        }
        "text/plain" | "text/html" => {
            return;
        }
        _ => {}
    }

    if mime.starts_with("image/") || mime.starts_with("application/") {
        if let Some(att) = build_attachment(parsed) {
            attachments.push(att);
        }
        return;
    }

    if !parsed.subparts.is_empty() {
        for sub in &parsed.subparts {
            extract_recursive(sub, text, html, attachments);
        }
    }
}

fn build_attachment(parsed: &ParsedMail) -> Option<Attachment> {
    let content_id = parsed.headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("Content-ID"))
        .map(|h| strip_angles(h.get_value().trim()));

    let content_type = parsed.ctype.mimetype.clone();
    let filename = parsed
        .ctype
        .params
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("name"))
        .map(|(_, v)| v.clone());

    let disposition = parsed.headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("Content-Disposition"))
        .map(|h| h.get_value().to_lowercase());

    let is_inline = content_id.is_some()
        || disposition.as_deref().map_or(false, |d| d.contains("inline"));

    let body = parsed.get_body_raw().ok()?;
    let bytes = decode_transfer(&parsed, &body);

    if bytes.is_empty() {
        return None;
    }

    Some(Attachment::new(
        content_id,
        content_type,
        filename,
        bytes,
        is_inline,
    ))
}

fn decode_transfer(parsed: &ParsedMail, body: &[u8]) -> Vec<u8> {
    let te = parsed
        .headers
        .iter()
        .find(|h| h.get_key().eq_ignore_ascii_case("Content-Transfer-Encoding"))
        .map(|h| h.get_value().trim().to_lowercase())
        .unwrap_or_default();

    match te.as_str() {
        "base64" => {
            let s = String::from_utf8_lossy(body);
            let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
            match base64_decode(&cleaned) {
                Some(d) => d,
                None => body.to_vec(),
            }
        }
        "quoted-printable" => {
            let s = String::from_utf8_lossy(body);
            decode_quoted_printable(&s)
        }
        _ => body.to_vec(),
    }
}

fn base64_decode(input: &str) -> Option<Vec<u8>> {
    let lookup: [i16; 256] = {
        let mut table = [-1i16; 256];
        for (i, c) in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".iter().enumerate() {
            table[*c as usize] = i as i16;
        }
        table
    };
    let bytes: Vec<u8> = input
        .bytes()
        .filter(|b| *b != b'=' && !b.is_ascii_whitespace())
        .collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    for b in bytes {
        let v = lookup[b as usize];
        if v < 0 {
            return None;
        }
        buf = (buf << 6) | (v as u32);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8 & 0xff);
        }
    }
    Some(out)
}

fn decode_quoted_printable(input: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'=' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            if i + 2 < bytes.len() && bytes[i + 1] == b'\r' && bytes[i + 2] == b'\n' {
                i += 3;
                continue;
            }
            if i + 2 < bytes.len() {
                let hi = hex_val(bytes[i + 1]);
                let lo = hex_val(bytes[i + 2]);
                if let (Some(h), Some(l)) = (hi, lo) {
                    out.push((h << 4) | l);
                    i += 3;
                    continue;
                }
            }
            i += 1;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn strip_angles(s: &str) -> String {
    let t = s.trim();
    if t.starts_with('<') && t.ends_with('>') && t.len() > 1 {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

fn rewrite_cid_links(email: &mut Email, attachments: &[Attachment]) {
    let Some(html) = email.body_html.as_mut() else { return };
    for att in attachments {
        let Some(cid) = &att.cid else { continue };
        let target = format!("/api/attachments/{}/{}", email.id, cid);
        let patterns = [
            format!("cid:{}", cid),
            format!("cid:'{}'", cid),
            format!("cid:\"{}\"", cid),
        ];
        for p in patterns {
            *html = html.replace(&p, &target);
        }
    }
}
