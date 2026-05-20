# Quickstart: Dept Tracker (Debt Dashboard)

**Feature**: `001-debt-dashboard` | **Date**: 2026-05-20

## Prerequisites

- Docker 24+ and Docker Compose v2
- ~512 MB RAM for the container
- Port `8080` available on the host (configurable)

## Run with Docker Compose

```bash
git clone <repository-url>
cd dept_tracker
docker compose -f docker/compose.yml up -d
```

Open **http://localhost:8080** in a browser.

Data persists in `./data/dept_tracker.db` (mounted volume). The SQLite file is internal; use **Export JSON** in the app for portable backups.

## First-use flow

1. Open the dashboard → empty state → **Add loan**.
2. Choose **Quick** (existing loan) or **Advanced** (with start date and optional payment history).
3. Enter name, remaining balance, and either fixed payment or **APR %**.
4. Save → loan appears with payoff projection and monthly obligation.
5. Expand a loan → record payments, add **Sonderzahlung (extra payment)**, view interest summary.

## Backup & restore

- **Export**: Settings → **Export JSON** → save file.
- **Import**: Settings → **Import JSON** → confirm **full replace** (existing data overwritten).

## Security (no login)

The app has **no authentication**. Anyone who can open the URL can read and change data.

- Prefer `127.0.0.1` binding on shared hosts, or
- Place behind a VPN / reverse proxy with auth, or
- Restrict firewall to your LAN only.

Do not expose port 8080 to the public internet without an external auth layer.

## Proxmox

1. Create an LXC or VM with Docker.
2. Clone repo or pull image from registry (when published).
3. Run compose with volume on persistent disk.
4. Optional: reverse proxy (Traefik/Caddy) with TLS on your LAN hostname.

## Umbrel

1. Install via custom Docker app or Community Store manifest (when published).
2. Map port `8080` and persistent volume `/data`.
3. Access from Umbrel dashboard URL.

## Development (after implementation)

```bash
# Backend
cd backend && cargo run

# Frontend (dev server with proxy)
cd frontend && npm install && npm run dev

# Tests
cargo test --workspace
cd frontend && npm test
```

## Troubleshooting

| Issue | Action |
|-------|--------|
| Blank dashboard after restart | Check volume mount; inspect logs `docker compose logs` |
| Wrong payoff date | Verify APR vs fixed payment mode; check scheduled Sonderzahlungen |
| Import failed | Ensure `schema_version: 1` and confirm replace dialog |
