CREATE TABLE IF NOT EXISTS addresses (
    id          TEXT PRIMARY KEY,
    address     TEXT NOT NULL UNIQUE,
    domain      TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    expires_at  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS emails (
    id            TEXT PRIMARY KEY,
    to_address    TEXT NOT NULL,
    from_addr     TEXT NOT NULL,
    subject       TEXT NOT NULL DEFAULT '',
    body_text     TEXT,
    body_html     TEXT,
    raw           TEXT,
    received_at   INTEGER NOT NULL,
    is_read       INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_emails_to_address ON emails(to_address);
CREATE INDEX IF NOT EXISTS idx_emails_received_at ON emails(received_at);
CREATE INDEX IF NOT EXISTS idx_addresses_expires_at ON addresses(expires_at);
