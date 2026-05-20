# Tasks: Debt Dashboard & Loan Tracking

**Input**: Design documents from `/specs/001-debt-dashboard/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/openapi.yaml, quickstart.md

**Tests**: Mandatory per constitution (Principle IV) and plan.md — test tasks precede implementation in each user story phase.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: US1–US5 maps to spec.md user stories

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Initialize Rust workspace, Svelte frontend, Docker, and CI skeleton.

- [x] T001 Create repository layout per plan.md: `backend/`, `frontend/`, `docker/`, `.github/workflows/`
- [x] T002 Initialize Rust workspace in `backend/Cargo.toml` with crates `domain`, `db`, `api`
- [x] T003 [P] Scaffold `backend/crates/domain/Cargo.toml` and `src/lib.rs` with public module stubs
- [x] T004 [P] Scaffold `backend/crates/db/Cargo.toml` and `src/lib.rs` with sqlx SQLite pool helper
- [x] T005 [P] Scaffold `backend/crates/api/Cargo.toml` with Axum, tower-http, and binary `src/main.rs`
- [x] T006 Initialize Svelte 5 + Vite + TypeScript project in `frontend/package.json` and `frontend/vite.config.ts`
- [x] T007 [P] Add root `.gitignore` entries for `target/`, `node_modules/`, `frontend/build/`, `data/`
- [x] T008 [P] Create `docker/Dockerfile` multi-stage skeleton (Rust build, Node build, runtime)
- [x] T009 [P] Create `docker/compose.yml` with port 8080 and volume `./data:/data`
- [x] T010 [P] Create `.github/workflows/ci.yml` running `cargo test`, `cargo clippy`, `npm test` placeholders

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Database schema, shared domain types, API shell, frontend shell — MUST complete before user stories.

**⚠️ CRITICAL**: No user story work until this phase is complete.

- [x] T011 Create SQL migration `backend/migrations/001_initial.sql` for `settings`, `loans`, `recurring_sonderzahlungen`, `scheduled_sonderzahlungen`, `payment_events` per data-model.md
- [x] T012 Implement migration runner and SQLite path from `DATA_DIR` env in `backend/crates/db/src/migrate.rs`
- [x] T013 [P] Implement money and ID types in `backend/crates/domain/src/money.rs` and `backend/crates/domain/src/types.rs`
- [x] T014 [P] Implement loan input validation (mutually exclusive fixed/APR, positive amounts) in `backend/crates/domain/src/validation.rs`
- [x] T015 Implement loan row model and repository CRUD in `backend/crates/db/src/loans.rs`
- [x] T016 [P] Implement settings singleton repository in `backend/crates/db/src/settings.rs`
- [x] T017 Wire Axum router with `/api/v1/health` and static file serving from `frontend/build` in `backend/crates/api/src/router.rs`
- [x] T018 Implement structured API errors and request tracing in `backend/crates/api/src/error.rs`
- [x] T019 [P] Create API client base and types in `frontend/src/lib/api/client.ts`
- [x] T020 [P] Create currency/date formatters in `frontend/src/lib/format.ts`
- [x] T021 [P] Create app shell layout and global styles in `frontend/src/app.css` and `frontend/src/routes/+layout.svelte`
- [x] T022 Add test database fixture helper using temp SQLite file in `backend/crates/db/src/test_support.rs`

**Checkpoint**: Foundation ready — user story phases may begin.

---

## Phase 3: User Story 1 - Dashboard at a Glance (Priority: P1) 🎯 MVP

**Goal**: List active loans with balances, periodic payment, last payment, payoff projection, household totals, empty state, archived toggle.

**Independent Test**: Seed loans via test helper; `GET /api/v1/dashboard` returns correct summaries; UI shows list and totals without add-loan flow.

### Tests for User Story 1 (MANDATORY) ⚠️

> **Write these tests FIRST, ensure they FAIL before implementation.**

- [x] T023 [P] [US1] Unit tests for household totals and monthly normalization in `backend/crates/domain/src/projection.rs` and `backend/crates/domain/tests/dashboard_totals.rs`
- [x] T024 [P] [US1] Integration test for `GET /api/v1/dashboard` in `backend/crates/api/tests/dashboard_test.rs`
- [x] T025 [P] [US1] Contract test asserting dashboard response shape against `specs/001-debt-dashboard/contracts/openapi.yaml` in `backend/crates/api/tests/contract_dashboard.rs`

### Implementation for User Story 1

- [x] T026 [US1] Implement payoff projection simulation (forward periods until balance ≤ 0) in `backend/crates/domain/src/projection.rs`
- [x] T027 [US1] Implement dashboard aggregate builder (loan summaries + household totals) in `backend/crates/domain/src/dashboard.rs`
- [x] T028 [US1] Implement `list_active_loans` and last-payment lookup in `backend/crates/db/src/loans.rs`
- [x] T029 [US1] Implement `GET /api/v1/dashboard` handler in `backend/crates/api/src/routes/dashboard.rs`
- [x] T030 [P] [US1] Create `LoanCard.svelte` expandable row in `frontend/src/components/LoanCard.svelte`
- [x] T031 [P] [US1] Create `HouseholdSummary.svelte` in `frontend/src/components/HouseholdSummary.svelte`
- [x] T032 [US1] Implement dashboard page with empty state and archived toggle in `frontend/src/routes/+page.svelte`
- [x] T033 [US1] Playwright E2E test: dashboard loads seeded loans in `frontend/tests/e2e/dashboard.spec.ts`

**Checkpoint**: User Story 1 fully functional and independently testable.

---

## Phase 4: User Story 2 - Add a New Loan (Priority: P2)

**Goal**: Add-loan modal with Quick/Advanced modes, APR or fixed payment, frequency, recurring yearly Sonderzahlungen.

**Independent Test**: Complete add-loan form; new loan appears on dashboard with correct payment and payoff projection.

### Tests for User Story 2 (MANDATORY) ⚠️

- [x] T034 [P] [US2] Unit tests for APR-derived periodic payment in `backend/crates/domain/tests/apr_payment.rs`
- [x] T035 [P] [US2] Integration tests for `POST /api/v1/loans` Quick and Advanced modes in `backend/crates/api/tests/loans_create_test.rs`

### Implementation for User Story 2

- [x] T036 [US2] Implement APR annuity payment calculation in `backend/crates/domain/src/amortization.rs`
- [x] T037 [US2] Implement create-loan orchestration (Quick vs Advanced, optional backfill) in `backend/crates/db/src/loans_create.rs`
- [x] T038 [US2] Implement recurring Sonderzahlung repository in `backend/crates/db/src/recurring_sonderzahlungen.rs`
- [x] T039 [US2] Implement `POST /api/v1/loans` in `backend/crates/api/src/routes/loans.rs`
- [x] T040 [P] [US2] Create `AddLoanModal.svelte` with Quick/Advanced tabs in `frontend/src/components/AddLoanModal.svelte`
- [x] T041 [P] [US2] Create `RecurringSonderzahlungFields.svelte` in `frontend/src/components/RecurringSonderzahlungFields.svelte`
- [x] T042 [US2] Wire “Add loan” button and modal submit to API in `frontend/src/routes/+page.svelte`
- [x] T043 [US2] Playwright E2E: add Quick loan and verify dashboard row in `frontend/tests/e2e/add-loan.spec.ts`

**Checkpoint**: User Stories 1 and 2 work independently.

---

## Phase 5: User Story 3 - Extra Payments on a Loan (Priority: P3)

**Goal**: Immediate and scheduled Sonderzahlungen; upcoming list; edit/cancel pending; overpayment warning.

**Independent Test**: Expand loan, add immediate and scheduled extra payments; balance and payoff projection update.

### Tests for User Story 3 (MANDATORY) ⚠️

- [x] T044 [P] [US3] Unit tests for balance reduction and projection with extras in `backend/crates/domain/tests/sonderzahlung_projection.rs`
- [x] T045 [P] [US3] Integration tests for immediate and scheduled endpoints in `backend/crates/api/tests/sonderzahlungen_test.rs`

### Implementation for User Story 3

- [x] T046 [US3] Implement payment event writer with interest/principal split in `backend/crates/domain/src/payment_split.rs`
- [x] T047 [US3] Implement `payment_events` repository in `backend/crates/db/src/payment_events.rs`
- [x] T048 [US3] Implement `scheduled_sonderzahlungen` repository in `backend/crates/db/src/scheduled_sonderzahlungen.rs`
- [x] T049 [US3] Implement `POST .../sonderzahlungen/immediate` and scheduled CRUD routes in `backend/crates/api/src/routes/sonderzahlungen.rs`
- [x] T050 [P] [US3] Create `SonderzahlungForm.svelte` (inline “extra payment” label) in `frontend/src/components/SonderzahlungForm.svelte`
- [x] T051 [P] [US3] Create `UpcomingPayments.svelte` in `frontend/src/components/UpcomingPayments.svelte`
- [x] T052 [US3] Integrate extra-payment actions into expanded `LoanCard.svelte`
- [x] T053 [US3] Playwright E2E: record and schedule Sonderzahlung in `frontend/tests/e2e/sonderzahlung.spec.ts`

**Checkpoint**: User Stories 1–3 independently functional.

---

## Phase 6: User Story 4 - Interest Paid and Remaining Interest (Priority: P4)

**Goal**: Show interest paid to date and estimated remaining interest in loan detail; handle missing rate gracefully.

**Independent Test**: Known amortization fixture; expand loan; values match domain expectations.

### Tests for User Story 4 (MANDATORY) ⚠️

- [x] T054 [P] [US4] Unit tests for interest-to-date and remaining-interest sums in `backend/crates/domain/tests/interest_summary.rs`
- [x] T055 [P] [US4] Integration test for interest fields on `GET /api/v1/loans/{id}` in `backend/crates/api/tests/loan_detail_test.rs`

### Implementation for User Story 4

- [x] T056 [US4] Implement interest summary calculator in `backend/crates/domain/src/interest.rs`
- [x] T057 [US4] Extend loan detail DTO and `GET /api/v1/loans/{loanId}` in `backend/crates/api/src/routes/loans.rs`
- [x] T058 [P] [US4] Create `InterestSummary.svelte` with missing-data message in `frontend/src/components/InterestSummary.svelte`
- [x] T059 [US4] Show `InterestSummary` in expanded loan detail in `frontend/src/components/LoanCard.svelte`
- [x] T060 [US4] Playwright E2E: interest section visible for APR loan in `frontend/tests/e2e/interest.spec.ts`

**Checkpoint**: User Stories 1–4 independently functional.

---

## Phase 7: User Story 5 - Manage Loans and Payment History (Priority: P5)

**Goal**: Edit loan, record regular payments, history view, archive, delete, JSON export/import.

**Independent Test**: Edit loan, record payment, export JSON, import JSON round-trip, archive loan.

### Tests for User Story 5 (MANDATORY) ⚠️

- [x] T061 [P] [US5] Integration tests for patch, payments list, archive, delete in `backend/crates/api/tests/loans_manage_test.rs`
- [x] T062 [P] [US5] Integration test for export/import round-trip in `backend/crates/api/tests/export_import_test.rs`

### Implementation for User Story 5

- [x] T063 [US5] Implement `PATCH /api/v1/loans/{loanId}` in `backend/crates/api/src/routes/loans.rs`
- [x] T064 [US5] Implement `GET/POST /api/v1/loans/{loanId}/payments` in `backend/crates/api/src/routes/payments.rs`
- [x] T065 [US5] Implement `POST /api/v1/loans/{loanId}/archive` in `backend/crates/api/src/routes/loans.rs`
- [x] T066 [US5] Implement JSON export bundle builder in `backend/crates/db/src/export.rs`
- [x] T067 [US5] Implement full-replace import with transaction in `backend/crates/db/src/import.rs`
- [x] T068 [US5] Implement `GET /api/v1/export` and `POST /api/v1/import` in `backend/crates/api/src/routes/backup.rs`
- [x] T069 [P] [US5] Create `PaymentHistory.svelte` in `frontend/src/components/PaymentHistory.svelte`
- [x] T070 [P] [US5] Create `EditLoanModal.svelte` in `frontend/src/components/EditLoanModal.svelte`
- [x] T071 [P] [US5] Create settings panel with export/import in `frontend/src/components/SettingsPanel.svelte`
- [x] T072 [US5] Wire record-payment, archive, delete, and settings actions in `frontend/src/components/LoanCard.svelte` and `frontend/src/routes/+page.svelte`
- [x] T073 [US5] Playwright E2E: export/import round-trip in `frontend/tests/e2e/backup.spec.ts`

**Checkpoint**: All user stories independently functional.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Docker production build, accessibility, progress indicator, docs, performance validation.

- [x] T074 [P] Finalize production `docker/Dockerfile` copying `frontend/build` and setting `DATA_DIR=/data`
- [x] T075 Complete `specs/001-debt-dashboard/quickstart.md` validation against real compose commands
- [x] T076 [P] Add per-loan progress percent to dashboard DTO and `LoanCard.svelte` (FR-020)
- [x] T077 [P] Accessibility pass: focus trap in modals, keyboard navigation, contrast in `frontend/src/app.css`
- [x] T078 Add Vitest unit tests for formatters in `frontend/src/lib/format.test.ts`
- [x] T079 Run performance smoke: 20-loan dashboard <2s documented in `specs/001-debt-dashboard/plan.md` verification notes
- [x] T080 Enable CI gates: fail on `cargo test`, `cargo clippy -D warnings`, Playwright smoke in `.github/workflows/ci.yml`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately.
- **Foundational (Phase 2)**: Depends on Setup — **blocks all user stories**.
- **User Stories (Phases 3–7)**: All depend on Foundational completion.
  - US2 depends on US1 dashboard API (extends same routes file) but is independently testable via `POST /loans`.
  - US3 depends on loans existing (US2 or seed data).
  - US4 depends on payment events from US3 (or Advanced backfill in US2).
  - US5 depends on core loan/payment persistence (US2–US3).
- **Polish (Phase 8)**: Depends on desired user stories being complete (minimum US1–US3 for MVP demo; all US1–US5 for full feature).

### User Story Dependencies

| Story | Depends on | Independent test via |
|-------|------------|----------------------|
| US1 | Foundational | Seeded DB + `GET /dashboard` + E2E |
| US2 | Foundational, US1 API shell | `POST /loans` + E2E |
| US3 | US2 loan exists | Sonderzahlung endpoints + E2E |
| US4 | US3 payment history (or US2 Advanced backfill) | `GET /loans/{id}` interest fields |
| US5 | US2–US3 data paths | export/import + manage endpoints |

### Within Each User Story

- Tests MUST fail before implementation (Red-Green-Refactor).
- Domain logic before DB repositories before API routes before UI.
- Story checkpoint before moving to next priority.

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel after T002.
- Foundational: T013/T014/T016/T019/T020/T021 in parallel after T011–T012.
- Within a story: tests marked [P] in parallel; UI components marked [P] in parallel after API ready.

---

## Parallel Example: User Story 1

```bash
# Tests first (parallel):
T023: backend/crates/domain/tests/dashboard_totals.rs
T024: backend/crates/api/tests/dashboard_test.rs
T025: backend/crates/api/tests/contract_dashboard.rs

# UI components (parallel after T029):
T030: frontend/src/components/LoanCard.svelte
T031: frontend/src/components/HouseholdSummary.svelte
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup  
2. Complete Phase 2: Foundational (**CRITICAL**)  
3. Complete Phase 3: User Story 1  
4. **STOP and VALIDATE**: `cargo test`, dashboard E2E, manual Docker smoke  
5. Demo/deploy if ready  

### Incremental Delivery

1. Setup + Foundational → Foundation ready  
2. US1 → Dashboard MVP  
3. US2 → Add loans  
4. US3 → Extra payments  
5. US4 → Interest insights  
6. US5 → Manage + backup  
7. Polish → Production Docker + CI + a11y  

### Suggested MVP Scope

**User Story 1 only** (Phases 1–3) after Foundational — delivers immediate value as read-only dashboard with seeded or migrated data; pair with US2 next for full onboarding.

---

## Task Summary

| Phase | Task IDs | Count |
|-------|----------|-------|
| Setup | T001–T010 | 10 |
| Foundational | T011–T022 | 12 |
| US1 Dashboard | T023–T033 | 11 |
| US2 Add loan | T034–T043 | 10 |
| US3 Sonderzahlung | T044–T053 | 10 |
| US4 Interest | T054–T060 | 7 |
| US5 Manage | T061–T073 | 13 |
| Polish | T074–T080 | 7 |
| **Total** | **T001–T080** | **80** |

**Format validation**: All 80 tasks use `- [ ] [TaskID] [P?] [Story?] Description with file path` format.
