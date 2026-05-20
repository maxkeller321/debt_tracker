# Dept Tracker

Self-hosted debt dashboard for home servers (Proxmox, Umbrel, etc.). Track multiple loans, regular payments, Sonderzahlungen (extra payments), interest, and payoff projections. **No login** — you secure access on the network.

## Quick start

```bash
docker compose -f docker/compose.yml up -d --build
```

Open http://localhost:8080 — add your first loan from the empty state.

Data lives in `./data/dept_tracker.db` (Docker volume). For portable backups use **Settings → Export JSON**; import does a **full replace** of all data.

Every loan needs an **interest rate (APR %)** so interest and payoff projections can be calculated (use `0` for interest-free loans). **Regular installments are applied automatically** on each due date; only **Sonderzahlungen** (extra payments) are entered manually. The UI is available in **English, German, Spanish, and French** (flag selector in the header).

> **Security:** No authentication. Do not expose port 8080 to the internet without a VPN or reverse proxy with auth. Prefer LAN-only or `127.0.0.1` on shared hosts.

More detail: [quickstart.md](specs/001-debt-dashboard/quickstart.md)

## Development

```bash
cd backend && DATA_DIR=../data cargo run -p api
cd frontend && npm install && npm run dev   # proxies /api → :8080
```

```bash
cd backend && cargo test
cd frontend && npm test && npm run test:e2e
```

## Stack

Rust (Axum, sqlx, SQLite) · Svelte 5 + Vite · Docker
