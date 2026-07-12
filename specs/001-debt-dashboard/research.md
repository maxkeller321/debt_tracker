# Research: Debt Dashboard & Loan Tracking

**Feature**: `001-debt-dashboard` | **Date**: 2026-05-20

## 1. Application stack (web UI + home server)

**Decision**: Rust backend (Axum) + Svelte 5 frontend (Vite), single Docker image.

**Rationale**:
- User requested high interactivity and previously suggested Rust; Axum + dedicated `domain` crate keeps loan math testable and fast.
- Clarification locked v1 to browser UI served from the container—not Tauri.
- Svelte gives a small bundle, reactive UI, and professional components without heavy SPA overhead; static assets served by Axum via `tower-http`.

**Alternatives considered**:
| Alternative | Rejected because |
|-------------|------------------|
| Tauri desktop | Conflicts with clarified browser + Docker delivery |
| Leptos full-stack Rust UI | Smaller ecosystem for form-heavy CRUD; steeper contributor curve |
| Next.js + Node API | Extra runtime; worse alignment with Rust performance goal |
| Python FastAPI + React | Adequate but not aligned with stated Rust preference |

## 2. Persistence

**Decision**: SQLite via `sqlx` with file at `/data/debt_tracker.db` (volume-mounted).

**Rationale**: User assumption; embedded DB fits single-household, no external DB ops, atomic writes, easy container volume backup alongside JSON export.

**Alternatives considered**: PostgreSQL (overkill for one home instance), JSON file as primary store (weak concurrent write guarantees).

## 3. Loan mathematics

**Decision**: Monthly simulation engine in `domain` crate with explicit rules:

- **APR mode**: Periodic rate `r = APR / periods_per_year`. Payment derived via standard annuity formula when term implied by paying until zero; for ongoing loans use user-visible periodic payment computed at save from remaining balance and APR.
- **Fixed payment mode**: Each period apply payment; interest portion = `balance * r` (if APR known) or allocate from payment history inference; principal = `payment - interest`.
- **Projections**: Forward-simulate period-by-period including recurring yearly Sonderzahlungen (on configured month/day) and pending scheduled Sonderzahlungen until `balance <= 0` → payoff date.
- **Interest paid to date**: Sum interest portions from all recorded payment events (backfill + manual).
- **Remaining interest**: Sum projected interest portions from simulation start (today) to payoff.

**Rationale**: Spec requires APR derivation, Sonderzahlungen in projections, and audit trail; period simulation handles irregular extras better than closed-form alone.

**Alternatives considered**: External amortization library only (insufficient for Sonderzahlung scheduling); spreadsheet-style simple subtraction (fails interest reporting).

## 4. Scheduled Sonderzahlungen execution

**Decision**: v1 does **not** auto-post payments on due date. Scheduled items affect **projections** only; user records actual payment manually (or confirms prompt when opening loan with due scheduled items—UI enhancement in implement phase).

**Rationale**: Clarification deferred; manual recording matches “tracker not bank” assumption and avoids background cron in container.

## 5. JSON backup / restore

**Decision**: Export schema version `1`; import uses **full replace** (clear household data, load JSON) with confirmation dialog.

**Rationale**: Clarification chose JSON-only; full replace simplest for round-trip SC-006 and avoids merge conflicts.

## 6. Security

**Decision**: No in-app auth (per clarification). Document operator guidance: bind `127.0.0.1` or place behind reverse proxy/VPN; never port-forward publicly without auth layer.

## 7. Testing strategy

**Decision**:
- **Unit**: `cargo test` on `domain` (amortization, projections, edge cases).
- **Integration**: Axum routes against in-memory SQLite / temp file DB.
- **Contract**: HTTP tests against OpenAPI document in `contracts/openapi.yaml`.
- **E2E**: Playwright against running container for P1–P3 smoke paths.
- **CI**: GitHub Actions — `cargo test`, `cargo clippy`, frontend `npm test`, Playwright on PR.

**Rationale**: Constitution Principle IV; domain-heavy app benefits from math unit tests plus API contract tests.

## 8. Deployment

**Decision**: Multi-stage Dockerfile (Rust build → Node build frontend → slim runtime). `docker-compose.yml` with volume `./data:/data` and port `8080:8080`.

**Umbrel/Proxmox**: Document generic Docker install in `quickstart.md`; Umbrel custom app manifest deferred to implement task (community pattern).

## 9. UI / UX

**Decision**: Single-page dashboard with expandable loan rows; modal dialogs for add/edit; neutral light theme; inline “Sonderzahlung (extra payment)” label; system font stack; WCAG AA contrast target.

**Locale**: English UI for v1 implementation; structure strings for future German (`de`) pass.
