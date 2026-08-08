# Magic Mail Backend — Deployment

## Environment variables

Read by `magic-mail` at startup.

| Variable | Required | Default | Description |
|---|---|---|---|
| `ALLOWED_DOMAINS` | ✅ yes | — | Comma-separated mail domains the server accepts (e.g. `temp.realblue.lol,od3n.online,od3n.info`). MX records for these domains must point to your SMTP host. |
| `PORT` | no | `3001` | HTTP API listen port. Render injects this automatically for web services. |
| `SMTP_PORT` | no | `2525` | SMTP receiver listen port. |
| `DATABASE_URL` | no | `sqlite:tempmail.db?mode=rwc` | SQLite connection string. On Render, set to `sqlite:/var/data/tempmail.db?mode=rwc` and attach a Persistent Disk at `/var/data`. |
| `SMTP_HOSTNAME` | no | `tmpml.net` | Hostname advertised in SMTP `220`/`EHLO` responses. Set to your MX hostname. |
| `EMAIL_TTL_MINUTES` | no | `10` | Lifetime of generated temp addresses in minutes. |

## Render deployment (HTTP API + SQLite)

`render.yaml` defines a single **Web Service** running the Rust binary in Docker.

- HTTP API is public on `$PORT` (Render gives you an `https://<service>.onrender.com` URL).
- SQLite lives on a 1 GB Persistent Disk at `/var/data` so addresses & emails survive redeploy.
- Health check: `/api/health`.
- Set `ALLOWED_DOMAINS` and `SMTP_HOSTNAME` in the Render dashboard (marked `sync: false`).

### ⚠️ Important: SMTP on Render

Render **Web Services only expose one public port (HTTP on `$PORT`)**. The SMTP receiver
still runs inside the container on `SMTP_PORT` (default 2525), but it is **not reachable
from the internet** on a Render Web Service. Render also blocks port 25 unconditionally.

To actually receive inbound mail from the internet, pick one:

1. **VPS for SMTP (simplest, recommended)** — run the same binary on a VPS (Oracle Cloud
   free tier, DigitalOcean, etc.) where port 25 is allowed. Set `SMTP_PORT=25` and
   `DATABASE_URL` to a path on the VPS disk. Point the MX records of your allowed
   domains to the VPS. Run HTTP on Render, SMTP on the VPS — but then they use
   **separate SQLite databases**, so this only works if you migrate the DB to
   **Postgres** (see option 3).

2. **Everything on one VPS** — run both HTTP and SMTP on a single VPS with port 25 +
   443 public. SQLite on local disk. Frontend still on Vercel, calling the VPS HTTPS
   endpoint. This is what the original README intended.

3. **Migrate to Postgres** — use Render's managed Postgres so the HTTP service (Render)
   and an SMTP service (Render TCP Service or VPS) can share one database. Requires
   switching `sqlx` to the `postgres` feature and adjusting the connection logic
   (not done in this commit).

For a temp-mail MVP, **option 2 (single VPS)** is the most reliable way to get real
inbound mail. Render is fine for the HTTP API if you don't need real inbound SMTP,
or if you only test via the local SMTP port.

## DNS

Point the MX record of each allowed domain to the host running the public SMTP server.
