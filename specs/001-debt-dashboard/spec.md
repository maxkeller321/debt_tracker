# Feature Specification: Debt Dashboard & Loan Tracking

**Feature Branch**: `001-debt-dashboard`  
**Created**: 2026-05-20  
**Status**: Draft  
**Input**: User description: "Simple debt tracking app with dashboard showing all loans, monthly rates, last payment dates, payoff projections; add-loan flow with amount/rate/schedule and recurring yearly Sonderzahlungen; per-loan extra payments and interest views; local storage; fast interactive UI; Docker deploy for Proxmox/Umbrel."

## Clarifications

### Session 2026-05-20

- Q: When a user chooses a percentage instead of a fixed payment amount, what does that percentage represent? → A: Annual interest rate (APR); the system derives periodic payment and amortization from APR, remaining balance, and payment frequency.
- Q: How should access control work on the home server? → A: No authentication; anyone who can reach the application URL can use it.
- Q: How should "add loan" handle existing debt already in progress? → A: Quick and Advanced modes — Quick uses current remaining balance and terms only; Advanced adds loan start date and optional past payment backfill.
- Q: Which application delivery model should v1 target? → A: Web UI in browser — the container serves the app; users open it via URL.
- Q: What backup and restore format should v1 support? → A: JSON only — human-readable export/import; the internal database file is not exposed to operators.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Dashboard at a Glance (Priority: P1)

As a household user, I open the app and immediately see every active loan on one screen with the information I need to plan cash flow this month.

**Why this priority**: Without a trustworthy overview, the app delivers no value; every other feature depends on this view.

**Independent Test**: Can be fully tested by creating sample loans (via test data or setup) and verifying the dashboard lists each loan with balance, payment rate, last payment, and projected payoff date—without using add-loan or detail flows in the same test session if loans are pre-seeded.

**Acceptance Scenarios**:

1. **Given** the user has one or more active loans, **When** they open the home dashboard, **Then** they see a list of all loans showing at minimum: loan name/label, remaining balance, current periodic payment amount, payment frequency (monthly or yearly), date of last recorded payment, and estimated payoff date if the current payment plan continues unchanged.
2. **Given** multiple active loans exist, **When** the dashboard loads, **Then** the user also sees household totals: combined remaining balance and combined periodic payment obligation for the current month (or current year segment, normalized for display).
3. **Given** no loans exist yet, **When** the user opens the dashboard, **Then** they see a friendly empty state explaining what the app does and a clear control to add their first loan.
4. **Given** a loan is marked paid off or archived, **When** the user views the dashboard, **Then** that loan is hidden from the active list by default with an option to show completed/archived loans.

---

### User Story 2 - Add a New Loan (Priority: P2)

As a user, I add a new loan so the dashboard reflects another debt I am paying down.

**Why this priority**: Adding loans is the primary onboarding action after viewing the empty dashboard.

**Independent Test**: Can be tested by tapping "Add loan," completing the form, and confirming the new loan appears on the dashboard with correct payment terms—without using Sonderzahlung or interest detail features.

**Acceptance Scenarios**:

1. **Given** the user is on the dashboard, **When** they choose to add a loan, **Then** a dedicated add-loan dialog opens without losing dashboard context, defaulting to **Quick** mode.
2. **Given** **Quick** mode is selected, **When** the user enters a loan name, current remaining balance, and either a fixed periodic payment or APR (one method active), **Then** the system saves the loan with interest-paid-to-date starting at zero and projections from the current balance forward.
3. **Given** the user switches to **Advanced** mode, **When** they additionally provide loan start date and optionally backfill past regular payments and Sonderzahlungen, **Then** interest-paid-to-date and last-payment date reflect that history from the first recorded event.
4. **Given** the add-loan dialog is open in either mode, **When** the user sets payment frequency to monthly or yearly, **Then** all dashboard and projection calculations use that frequency consistently for that loan.
5. **Given** the user configures optional yearly recurring Sonderzahlungen, **When** they specify amount and timing rule (e.g., each December), **Then** those payments are included in payoff and interest projections from the effective loan timeline (start date in Advanced, save date in Quick).
6. **Given** valid loan details in the selected mode, **When** the user saves, **Then** the new loan appears on the dashboard with correct balance, rate, and projected payoff date.

---

### User Story 3 - Extra Payments on a Loan (Priority: P3)

As a user, I make or schedule one-off special payments (Sonderzahlungen) to reduce a loan faster than the base schedule.

**Why this priority**: Extra payments are a core differentiator for household debt planning and directly affect payoff dates shown on the dashboard.

**Independent Test**: Expand a single loan, add an immediate and a future-dated Sonderzahlung, and verify balance and projected payoff date update—without testing interest breakdown (Story 4) or add-loan form (Story 2).

**Acceptance Scenarios**:

1. **Given** a loan is expanded from the dashboard, **When** the user records a one-time Sonderzahlung today, **Then** the remaining balance decreases immediately, the event appears in payment history, and dashboard totals and payoff projection refresh.
2. **Given** a loan detail view is open, **When** the user schedules a Sonderzahlung for a future date and amount, **Then** it is stored, visible in an upcoming-payments list, and included in payoff projection from the scheduled date onward.
3. **Given** a scheduled Sonderzahlung has not yet occurred, **When** the user edits or cancels it, **Then** projections recalculate to match the change.
4. **Given** a Sonderzahlung would reduce balance below zero, **When** the user attempts to save, **Then** the system warns the user and requires explicit confirmation or adjustment before applying.

---

### User Story 4 - Interest Paid and Remaining Interest (Priority: P4)

As a user, I understand how much interest I have already paid on a loan and how much I can expect to pay until the loan ends at the current plan.

**Why this priority**: Interest visibility supports refinancing and payoff decisions; it builds on accurate balance and schedule data from earlier stories.

**Independent Test**: Open an expanded loan with known amortization inputs and verify displayed cumulative interest and projected remaining interest match documented calculation rules—without adding new loans or Sonderzahlungen in the same test.

**Acceptance Scenarios**:

1. **Given** a loan with payment history, **When** the user expands the loan, **Then** they see total interest paid to date based on recorded payments and the loan's rate terms.
2. **Given** the current payment plan (including recurring and scheduled Sonderzahlungen), **When** the user views the loan detail, **Then** they see estimated total interest still to be paid until projected payoff.
3. **Given** the user adds or changes a Sonderzahlung or edits loan terms, **When** they return to the loan detail, **Then** interest-to-date and remaining-interest estimates update consistently with the new plan.
4. **Given** insufficient data to compute interest (e.g., missing rate), **When** the user opens interest details, **Then** the app explains what information is needed instead of showing misleading numbers.

---

### User Story 5 - Manage Loans and Payment History (Priority: P5)

As a user, I keep loan information accurate over time by editing terms, recording payments, reviewing history, and retiring paid-off loans.

**Why this priority**: Real-world loans change; management features prevent the dashboard from becoming stale without re-entering data.

**Independent Test**: Edit an existing loan's name or rate, record a regular payment, view history, archive a paid-off loan, and export data—each verifiable without re-testing Sonderzahlung scheduling logic beyond history entries.

**Acceptance Scenarios**:

1. **Given** an active loan, **When** the user edits allowed fields (name, balance adjustments with reason, payment amount or percentage, frequency, recurring yearly Sonderzahlungen), **Then** changes persist and dashboard projections update.
2. **Given** a due regular payment, **When** the user records that payment as made (with date and amount), **Then** last-payment date updates, balance decreases, and history logs the event.
3. **Given** a loan detail view, **When** the user opens payment history, **Then** they see a chronological list of regular payments and Sonderzahlungen with dates and amounts.
4. **Given** a loan balance reaches zero, **When** the user marks it paid off, **Then** it moves to archived/completed state and no longer counts in active totals unless the user shows archived loans.
5. **Given** the user wants a safety copy, **When** they export data, **Then** they receive a portable JSON backup containing all loans and payment events; **When** they import that JSON, **Then** all loans and events are restored without data loss.
6. **Given** the user confirms deletion, **When** they delete a loan, **Then** it is removed from active data and no longer appears on the dashboard.

---

### Edge Cases

- What happens when the user enters zero or negative principal? → Block save with a clear message.
- What happens when fixed payment amount and APR are both empty or both filled? → Require exactly one method; explain in plain language.
- What happens when APR is 0%? → Treat as interest-free; periodic payment covers principal only per schedule rules.
- How does the system handle a yearly payment loan on a monthly-oriented dashboard total? → Normalize to a monthly equivalent for the "this month" summary with a tooltip or label explaining the conversion.
- What happens when last payment date is unknown for a new Quick-mode loan? → Show "No payments recorded yet" and use save/creation date only where needed for display, not fake history.
- What happens when Advanced backfill has gaps in payment dates? → Accept non-contiguous history; interest and balance calculations use only recorded events (no invented payments).
- What happens when scheduled Sonderzahlung date is in the past at entry time? → Treat as immediate payment or prompt user to confirm back-dated application.
- What happens after app restart? → All loans, schedules, and history reappear exactly as before (no data loss).
- What happens when disk is full or save fails? → Show error; do not partially update totals without persisting the underlying event.
- What happens when the app is reachable from untrusted networks? → Out of scope for the app; operators MUST restrict network access (firewall, VPN, reverse proxy) because the application provides no login.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a top-level dashboard listing all active loans with remaining balance, current periodic payment, payment frequency, last payment date, and projected payoff date at the current plan.
- **FR-002**: System MUST show aggregate active-loan totals for combined balance and combined periodic obligation on the dashboard.
- **FR-003**: System MUST provide a prominent control on the dashboard to add a new loan.
- **FR-004**: System MUST provide an add-loan flow with **Quick** (default) and **Advanced** modes. Quick collects: loan label, current remaining balance, payment as fixed periodic amount OR APR (mutually exclusive), payment frequency, and optional yearly recurring Sonderzahlungen. Advanced adds: loan start date and optional backfill of past regular payments and Sonderzahlungen. When APR is provided, the system MUST derive the periodic payment and amortization from APR, balance, and frequency.
- **FR-004a**: In Quick mode, interest-paid-to-date MUST start at zero until the user records payments in the app. In Advanced mode, interest-paid-to-date MUST reflect backfilled payment history.
- **FR-005**: System MUST validate add-loan and edit-loan inputs with user-readable error messages before saving.
- **FR-006**: System MUST allow expanding a loan from the dashboard to a detail view without leaving the overall dashboard context (e.g., expand-in-place or panel).
- **FR-007**: System MUST allow recording an immediate one-time Sonderzahlung from the loan detail view.
- **FR-008**: System MUST allow scheduling a future Sonderzahlung with date and amount, and support edit/cancel before execution.
- **FR-009**: System MUST include all configured recurring yearly Sonderzahlungen and scheduled Sonderzahlungen in payoff date projections.
- **FR-010**: System MUST display cumulative interest paid to date per loan in the detail view.
- **FR-011**: System MUST display estimated remaining interest until projected payoff per loan, based on the current plan including extras.
- **FR-012**: System MUST recalculate balances, projections, and interest figures when payments, Sonderzahlungen, or loan terms change.
- **FR-013**: System MUST persist all loans, schedules, payments, and Sonderzahlungen locally on the home server without requiring cloud services for core operation.
- **FR-014**: System MUST retain data across application restarts and container restarts when using the provided deployment package.
- **FR-015**: System MUST allow editing loan terms and recording regular payments with dated history entries.
- **FR-016**: System MUST provide chronological payment history per loan (regular + special payments).
- **FR-017**: System MUST support archiving or marking loans paid off and hiding them from default dashboard view.
- **FR-018**: System MUST support deleting a loan with explicit user confirmation.
- **FR-019**: System MUST support exporting and importing all household data as a single human-readable JSON file (single-household scope). Restore MUST replace or merge per documented import rules in operator docs; round-trip MUST preserve all loans and payment events. The internal database file MUST NOT be offered as an operator-facing backup mechanism in v1.
- **FR-020**: System MUST show per-loan progress indication (e.g., percent of original principal paid down) on dashboard or detail view.
- **FR-021**: System MUST respond to common interactions (open dashboard, expand loan, open add dialog, save loan) without noticeable delay under normal home-server load (see Success Criteria).
- **FR-022**: System MUST ship as a containerized web application: the container serves a browser-based UI reachable via URL, suitable for home-server platforms (e.g., Proxmox Docker host, Umbrel app catalog pattern), with operator documentation for install and data volume mounts.
- **FR-023**: System MUST use plain-language labels; define "Sonderzahlung" inline on first use as "extra payment" in the UI.
- **FR-024**: System MUST meet baseline accessibility on primary flows: readable contrast, visible focus, keyboard access to add/save/cancel and dashboard navigation.
- **FR-025**: System MUST NOT require user authentication; all features are available to any client that can reach the application URL.

### Key Entities

- **Loan**: A debt obligation; attributes include label, original principal (optional in Quick mode), current remaining balance, payment terms as either fixed periodic amount OR APR, payment frequency, loan start date (Advanced or derived), creation metadata, active vs archived status, optional notes, setup mode (quick vs advanced).
- **Payment schedule**: Derived rules for regular payments and recurring yearly Sonderzahlungen attached to a loan.
- **Payment event**: A recorded regular payment or executed Sonderzahlung with date, amount, and link to loan; drives last-payment date and balance.
- **Scheduled Sonderzahlung**: Future-dated planned extra payment; may be pending, executed, or cancelled.
- **Interest summary**: Computed views of interest paid to date and projected remaining interest for a loan under the current plan.
- **Household summary**: Aggregated totals across active loans for dashboard display.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new user can add their first loan and see it on the dashboard with correct payment and payoff information in under 3 minutes without reading external documentation.
- **SC-002**: With up to 20 active loans, the dashboard displays the full list and household totals within 2 seconds on typical home-server hardware.
- **SC-003**: Expanding a loan, opening the add-loan dialog, and saving a valid loan feel instantaneous (under 300 ms perceived response) for at least 95% of interactions in manual testing.
- **SC-004**: After recording a payment or Sonderzahlung, dashboard and detail figures update in the same session with no user-visible inconsistency between balance, last payment date, and payoff projection.
- **SC-005**: At least 90% of test participants (or structured walkthrough reviewers) correctly interpret dashboard fields (balance, monthly obligation, payoff date) without trainer hints.
- **SC-006**: JSON backup export and import round-trip preserves 100% of loans and payment events in acceptance testing.
- **SC-007**: Container deployment completes from documented steps on a fresh home-server test environment in under 15 minutes by an operator familiar with Docker.
- **SC-008**: Primary flows (dashboard, add loan, record payment, add Sonderzahlung) pass accessibility spot-check for contrast, focus visibility, and keyboard completion.

## Assumptions

- **Single household, no authentication**: One shared dataset per instance; no login, accounts, or roles. Any client that can reach the URL can view and modify data. Network isolation (LAN, VPN, firewall) is the operator's responsibility.
- **Single currency**: All amounts use one currency configured at instance level or implicit default; no live exchange rates.
- **Interest model**: When the user supplies APR, standard amortizing loan mathematics derive periodic payment, interest paid to date, and remaining interest. When the user supplies a fixed periodic payment instead, interest is inferred from payment history and schedule rules documented in the implementation plan. Estimates are labeled as projections when based on future scheduled events.
- **"Credit" in UI**: User-facing copy may say "loan" or "debt" in English; German operators may prefer "Kredit"—wording finalized in UI copy pass, behavior unchanged.
- **Local database**: Persistent store is local-only (user suggested SQLite); exact schema and stack are decided in `/speckit-plan`, not in this spec.
- **UI delivery**: v1 is a browser-based web UI served from the Docker container (not a native desktop-only shell). Implementation stack (e.g., Rust backend + web frontend) is decided in `/speckit-plan` as long as Success Criteria for responsiveness are met.
- **Docker delivery**: One official image (or compose file) with volume mount for data directory; Umbrel/Proxmox specifics handled in deployment docs.
- **Backup**: v1 backup/restore is JSON export/import only; operators do not copy the internal database file. Container volume still persists data across restarts.
- **No payment automation**: The app tracks and projects; it does not initiate bank transfers or connect to open banking in this feature.
- **Recurring Sonderzahlungen**: Yearly recurrence only in v1; other cadences deferred unless added in a later spec.
- **Data retention**: Records kept until user deletes or archives; no automatic purge.
