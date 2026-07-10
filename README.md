# Magic Mail

A temporary email service that lets you receive emails at disposable addresses. Built with a Rust backend (Axum + SQLite) and a Next.js frontend.

## Architecture

```
Internet
    │
    ├── Port 25/2525 (SMTP) → Rust SMTP Receiver → SQLite
    │
    └── Port 3001 (HTTP) → Axum API Server
                              │
                              ├─ REST /api/*
                              └─ SSE  /sse/inbox/:address
                                       │
                              Next.js Frontend (Port 3000)
```

## Project Structure

```
magic-mail/
├── server/          # Rust backend (Axum + SMTP)
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── api/            # REST + SSE endpoints
│   │   ├── smtp/           # SMTP receiver
│   │   ├── db/             # SQLite queries
│   │   ├── models.rs       # Data types
│   │   ├── notify.rs       # SSE broadcast channel
│   │   └── tasks/          # Background cleanup
│   └── Cargo.toml
│
└── magic-client/    # Next.js frontend
    └── src/
        ├── app/            # Pages + layout
        ├── components/     # AddressBar, InboxList, EmailViewer
        ├── hooks/          # useInbox (SSE + polling)
        └── lib/            # API client
```

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 20+
- npm (or bun)

### 1. Start the Backend

```bash
cd server
cargo run
```

The backend starts on:
- **HTTP API**: `http://localhost:3001`
- **SMTP**: `localhost:2525`

### 2. Start the Frontend

```bash
cd magic-client
npm install
npm run dev
```

The frontend starts on `http://localhost:3000` and auto-proxies `/api/*` and `/sse/*` to the backend.

### 3. Use It

Open `http://localhost:3000` in your browser. A temporary email address is generated automatically on first visit. Send a test email:

```powershell
Send-MailMessage -From "test@example.com" `
    -To "YOUR_GENERATED_ADDRESS" `
    -Subject "Hello" `
    -Body "Testing the temp mail service" `
    -SmtpServer "127.0.0.1" `
    -Port 2525
```

The email appears in the inbox in real-time via SSE.

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/health` | Health check |
| `GET` | `/api/domains` | List allowed domains |
| `POST` | `/api/address/generate` | Create a temp address |
| `GET` | `/api/emails/:address` | List emails for an address |
| `GET` | `/api/emails/:address/:id` | Get a single email |
| `DELETE` | `/api/emails/:address/:id` | Delete one email |
| `DELETE` | `/api/emails/:address` | Clear all emails |
| `GET` | `/sse/inbox/:address` | SSE stream for real-time updates |

## Configuration

Edit `server/src/main.rs` to set allowed domains and ports:

```rust
let allowed_domains = vec!["realblue.lol".to_string()];
```

- **SMTP port**: defaults to `2525` (change in `smtp/mod.rs`)
- **HTTP port**: defaults to `3001` (change in `api/mod.rs`)
- **Email TTL**: defaults to `10` minutes (change in `api/address.rs`)

## Deployment

For production deployment on a VPS (e.g., Oracle Cloud):

1. Change SMTP port to `25` in `smtp/mod.rs` (requires root)
2. Point MX record to your server IP
3. Open port 25 in your firewall / security list
4. Build the backend: `cd server && cargo build --release`
5. Build the frontend: `cd magic-client && npm run build`
6. Use `npm run start` or `pm2`/`systemd` to run

## Tech Stack

- **Backend**: Rust, Axum, SQLite (sqlx), tokio
- **Frontend**: Next.js, React, TypeScript, Tailwind CSS
- **Real-time**: Server-Sent Events (SSE)
