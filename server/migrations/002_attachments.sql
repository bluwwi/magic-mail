CREATE TABLE IF NOT EXISTS attachments (
    id            TEXT PRIMARY KEY,
    email_id      TEXT NOT NULL,
    cid           TEXT,
    content_type  TEXT NOT NULL DEFAULT 'application/octet-stream',
    filename      TEXT,
    data          BLOB NOT NULL,
    inline        INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_attachments_email_id ON attachments(email_id);
CREATE INDEX IF NOT EXISTS idx_attachments_cid ON attachments(email_id, cid);
