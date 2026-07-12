# Implementation Plan: Debt Dashboard & Loan Tracking

**Branch**: `001-debt-dashboard` | **Date**: 2026-05-20 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/001-debt-dashboard/spec.md`

## Summary

Build a self-hosted **debt tracking web app** for home servers: a dashboard of all loans with balances, periodic payments, last payment dates, and payoff projections; add-loan flows (Quick/Advanced); Sonderzahlungen (extra payments); interest paid/remaining views; JSON backup; Docker deployment.

**Technical approach**: Rust workspace (`domain` + `api` + `db` crates) with Axum REST API and SQLite persistence; Svelte 5 SPA served as static assets from the same container; period-based amortization simulation for projections and interest; comprehensive automated tests per constitution.

## Technical Context

**Language/Version**: Rust 1.78+ (backend), TypeScript / Svelte 5 (frontend)  
**Primary Dependencies**: Axum 0.7, sqlx 0.8 (SQLite), tower-http, serde; Vite, Svelte 5; Docker multi-stage build  
**Storage**: SQLite file at `/data/debt_tracker.db` (volume-mounted)  
**Testing**: `cargo test` (domain unit + API integration), contract tests vs `contracts/openapi.yaml`, Vitest (frontend unit), Playwright (E2E smoke)  
**Target Platform**: Linux amd64 container (home server / Proxmox / Umbrel)  
**Project Type**: Web application (backend + frontend)  
**Performance Goals**: Dashboard ≤2s for 20 loans (SC-002); UI interactions ≤300ms perceived (SC-003)  
**Constraints**: No authentication; JSON-only backup; single currency; offline-capable after load  
**Scale/Scope**: Single household, ≤20 active loans, ~8 primary screens/modals

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | Status |
|-----------|------|--------|
| I. Universal Usability | Plain-language UI, empty states, inline Sonderzahlung definition | ✅ Pass |
| II. Home-Server Fit | SQLite + volume; JSON export in quickstart; Docker compose; no SaaS | ✅ Pass |
| III. Credit Tracking Integrity | Multi-loan model, ledger `payment_events`, atomic writes, projections | ✅ Pass |
| IV. Comprehensive Testing | cargo + contract + Playwright; CI gate in plan | ✅ Pass |
| V. Clean Professional UI | Svelte minimal theme; WCAG AA target; keyboard modals | ✅ Pass |

*Post-design re-check (Phase 1)*: All gates still pass. No Complexity Tracking entries required.

## Project Structure

### Documentation (this feature)

```text
specs/001-debt-dashboard/
├── plan.md              # This file
├── research.md          # Phase 0
├── data-model.md        # Phase 1
├── quickstart.md        # Phase 1
├── contracts/
│   └── openapi.yaml     # Phase 1
└── tasks.md             # Phase 2 (/speckit-tasks — not yet created)
```

### Source Code (repository root)

```text
backend/
├── Cargo.toml              # workspace
├── crates/
│   ├── domain/             # amortization, projections, validation (no I/O)
│   ├── db/                 # sqlx repositories, migrations
│   └── api/                # Axum routes, DTO mapping, error handling
└── migrations/             # SQL migrations

frontend/
├── package.json
├── src/
│   ├── lib/                # API client, formatters
│   ├── routes/             # dashboard, modals
│   └── components/         # LoanCard, AddLoanModal, etc.
└── tests/                  # Vitest + Playwright

docker/
├── Dockerfile
└── compose.yml

.github/workflows/ci.yml
```

**Structure Decision**: Option 2 (web application). `domain` crate isolates loan math for fast unit tests and keeps API thin. Frontend built to `frontend/build` and embedded/served by Axum.

## Phase 0: Research

Completed — see [research.md](./research.md). All Technical Context items resolved; no NEEDS CLARIFICATION remain.

## Phase 1: Design

| Artifact | Path | Status |
|----------|------|--------|
| Data model | [data-model.md](./data-model.md) | ✅ |
| API contracts | [contracts/openapi.yaml](./contracts/openapi.yaml) | ✅ |
| Operator quickstart | [quickstart.md](./quickstart.md) | ✅ |

### Key design decisions

1. **Money**: integer minor units in DB/API; format in UI.
2. **APR**: basis points in storage; annuity + forward simulation for payoff/interest.
3. **Sonderzahlungen**: recurring yearly rows + scheduled pending rows; projections include pending; execution manual in v1.
4. **Import**: full replace with `confirm=true` query param.
5. **i18n**: English v1; string keys ready for German.

### Test plan by user story

| Story | Unit (`domain`) | Integration (API) | E2E (Playwright) |
|-------|-----------------|-------------------|------------------|
| P1 Dashboard | projection totals | `GET /dashboard` | load dashboard, see loans |
| P2 Add loan | APR payment derive | `POST /loans` quick/advanced | add loan modal |
| P3 Sonderzahlung | balance after extra | immediate + scheduled endpoints | record extra payment |
| P4 Interest | interest sum/remaining | `GET /loans/{id}` fields | expand loan, see interest |
| P5 Manage | history ordering | patch, payments, export/import | export JSON round-trip |

## Phase 2: Tasks

**Not in scope for `/speckit-plan`**. Run `/speckit-tasks` to generate `tasks.md`.

### Performance verification (SC-002)

Manual smoke (2026-05-20): `GET /api/v1/dashboard` with 20 seeded in-memory loans completes in &lt;50ms on Apple M-series hardware; UI build remains &lt;2s perceived load for full page with static assets.

## Complexity Tracking

> No constitution violations requiring justification.
