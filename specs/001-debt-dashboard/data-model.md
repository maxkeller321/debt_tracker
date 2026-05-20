# Data Model: Debt Dashboard & Loan Tracking

**Feature**: `001-debt-dashboard` | **Date**: 2026-05-20

## Overview

Single-household SQLite schema. All monetary values stored as integer **minor units** (cents) to avoid float drift. Percentages stored as basis points (e.g., `375` = 3.75% APR) or decimal string in domain only during calculation.

## Entities

### `settings` (singleton row)

| Field | Type | Notes |
|-------|------|-------|
| `id` | INTEGER | Always `1` |
| `currency_code` | TEXT | ISO 4217, default `EUR` |
| `created_at` | TEXT | ISO 8601 UTC |

### `loans`

| Field | Type | Constraints |
|-------|------|-------------|
| `id` | TEXT | UUID PK |
| `label` | TEXT | NOT NULL, 1–120 chars |
| `status` | TEXT | `active`, `archived` |
| `setup_mode` | TEXT | `quick`, `advanced` |
| `original_principal_minor` | INTEGER | NULL allowed (Quick) |
| `remaining_balance_minor` | INTEGER | NOT NULL, ≥ 0 |
| `payment_frequency` | TEXT | `monthly`, `yearly` |
| `payment_type` | TEXT | `fixed`, `apr` |
| `fixed_payment_minor` | INTEGER | NULL if `payment_type = apr` |
| `apr_basis_points` | INTEGER | NULL if `payment_type = fixed`; 0 = interest-free |
| `loan_start_date` | TEXT | DATE; required Advanced, NULL Quick → use `created_at` for projections |
| `created_at` | TEXT | ISO 8601 UTC |
| `updated_at` | TEXT | ISO 8601 UTC |
| `archived_at` | TEXT | NULL unless archived |
| `notes` | TEXT | Optional |

**Derived (not stored)**: periodic payment (when APR), payoff date, interest paid/remaining, progress % — computed in `domain` from loan + related rows.

### `recurring_sonderzahlungen`

Yearly extra payments attached to a loan.

| Field | Type | Constraints |
|-------|------|-------------|
| `id` | TEXT | UUID PK |
| `loan_id` | TEXT | FK → `loans.id` ON DELETE CASCADE |
| `amount_minor` | INTEGER | NOT NULL, > 0 |
| `month` | INTEGER | 1–12 |
| `day` | INTEGER | 1–28 (cap 28 to avoid Feb issues) |
| `enabled` | BOOLEAN | DEFAULT true |

### `scheduled_sonderzahlungen`

One-off future (or back-dated) extras.

| Field | Type | Constraints |
|-------|------|-------------|
| `id` | TEXT | UUID PK |
| `loan_id` | TEXT | FK → `loans.id` ON DELETE CASCADE |
| `amount_minor` | INTEGER | NOT NULL, > 0 |
| `due_date` | TEXT | DATE |
| `status` | TEXT | `pending`, `executed`, `cancelled` |
| `executed_payment_id` | TEXT | NULL; FK → `payment_events.id` when executed |
| `created_at` | TEXT | ISO 8601 UTC |

### `payment_events`

Immutable ledger of applied payments (regular + executed Sonderzahlungen).

| Field | Type | Constraints |
|-------|------|-------------|
| `id` | TEXT | UUID PK |
| `loan_id` | TEXT | FK → `loans.id` ON DELETE CASCADE |
| `event_type` | TEXT | `regular`, `sonderzahlung` |
| `amount_minor` | INTEGER | NOT NULL, > 0 |
| `interest_portion_minor` | INTEGER | NOT NULL, ≥ 0 |
| `principal_portion_minor` | INTEGER | NOT NULL, ≥ 0 |
| `balance_after_minor` | INTEGER | NOT NULL, ≥ 0 |
| `paid_at` | TEXT | DATE |
| `note` | TEXT | Optional (e.g., balance adjustment reason) |
| `created_at` | TEXT | ISO 8601 UTC |

**Invariant**: `interest_portion_minor + principal_portion_minor = amount_minor`.

## State transitions

### Loan `status`

```text
active ──(mark paid off / archive)──► archived
archived ──(show on dashboard toggle)──► visible in UI only
active/archived ──(delete)──► removed (cascade children)
```

### Scheduled Sonderzahlung `status`

```text
pending ──(user records payment linked)──► executed
pending ──(user cancel)──► cancelled
pending ──(due_date < today at entry, user confirms)──► executed via immediate payment_event
```

## Validation rules (domain)

| Rule | Enforcement |
|------|-------------|
| Exactly one of `fixed_payment_minor` or `apr_basis_points` active per loan | On create/update |
| `remaining_balance_minor > 0` on create unless archiving | On create |
| Payment event amount ≤ remaining + tolerance | On record; warn if overpayment |
| Yearly dashboard total | `monthly_equivalent = yearly_payment / 12` for summary |
| Quick mode | No `payment_events` required at create; `interest_paid` = 0 |
| Advanced backfill | Events sorted by `paid_at`; recompute `remaining_balance_minor` from last event |

## JSON export schema (v1)

Top-level object:

```json
{
  "schema_version": 1,
  "exported_at": "ISO-8601",
  "currency_code": "EUR",
  "loans": [ /* loan + nested recurring_sonderzahlungen, scheduled_sonderzahlungen, payment_events */ ]
}
```

Import: validate `schema_version`, confirm with user, replace all tables in one transaction.

## Indexes

- `payment_events(loan_id, paid_at)`
- `scheduled_sonderzahlungen(loan_id, status, due_date)`
- `loans(status)`
