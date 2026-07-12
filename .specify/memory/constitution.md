<!--
Sync Impact Report
- Version change: (template placeholders) → 1.0.0
- Modified principles: N/A (initial ratification from template)
- Added sections: Core Principles (5), Product Scope & Constraints, Quality Gates, Governance
- Removed sections: Template example principles (Library-First, CLI Interface, etc.)
- Templates requiring updates:
  - ✅ .specify/templates/plan-template.md (Constitution Check gates)
  - ✅ .specify/templates/tasks-template.md (tests mandatory per constitution)
  - ✅ .specify/templates/spec-template.md (usability/accessibility guidance)
  - ✅ .cursor/rules/specify-rules.mdc (project context pointer)
  - ⚠ .specify/templates/checklist-template.md (no structural change needed)
- Follow-up TODOs: None
-->

# Debt Tracker Constitution

## Core Principles

### I. Universal Usability

The application MUST be understandable by users of all technical backgrounds.

- Use plain-language labels, help text, and error messages; avoid jargon unless defined in context.
- Primary flows (add a credit, record usage, view remaining balance) MUST be completable without reading external documentation.
- Empty states, confirmations, and errors MUST tell the user what happened and what to do next.
- New features MUST include acceptance scenarios written from a non-technical user perspective.

**Rationale**: Home-server operators range from hobbyists to power users; clarity is a core product requirement, not polish.

### II. Home-Server Fit

The system MUST remain simple to deploy and operate on a typical home server.

- Core credit tracking MUST work without mandatory third-party SaaS dependencies.
- Data MUST persist locally (file or embedded database) with predictable backup/restore documented in `quickstart.md`.
- Resource use MUST stay modest (single-instance deployment is acceptable for v1).
- Configuration MUST favor sensible defaults over extensive tuning.

**Rationale**: The product targets self-hosted environments where operational burden must stay low.

### III. Credit Tracking Integrity

Multiple credits and decreasing balances are the non-negotiable domain model.

- Users MUST be able to create and manage multiple independent credits (budgets/allowances).
- Each credit MUST display a current remaining balance that decreases when usage is recorded.
- Balance changes MUST be persisted atomically; restarts MUST NOT lose recorded usage.
- Usage history or an audit trail MUST be available so users can verify how a balance changed.
- Domain rules (e.g., cannot spend below zero unless explicitly specified) MUST be enforced in code and covered by tests.

**Rationale**: Accurate, trustworthy tracking of credits over time is the product's reason to exist.

### IV. Comprehensive Testing (NON-NEGOTIABLE)

Every behavior that affects users or persisted data MUST have automated test coverage.

- New behavior follows test-first discipline: write a failing test, implement, refactor (Red-Green-Refactor).
- Unit tests cover domain logic (balances, validation, edge cases).
- Integration tests cover persistence and API/UI flows end-to-end for each user story.
- CI (or local equivalent documented in the plan) MUST fail on test regressions before merge.
- Bug fixes MUST include a regression test unless technically infeasible (document why in the PR/plan).

**Rationale**: Reliability on a home server matters; the project owner requires everything tested.

### V. Clean Professional UI

The interface MUST be simple, consistent, and professional—not flashy or cluttered.

- Visual design uses a restrained palette, consistent spacing, and readable typography.
- Layouts MUST work on common desktop and mobile browser widths used at home.
- Primary flows MUST meet baseline accessibility: sufficient contrast, visible focus states, keyboard navigation for core actions.
- Decorative complexity and non-essential animations are out of scope unless a feature spec justifies them.

**Rationale**: Users asked for clean, professional presentation that still feels approachable.

## Product Scope & Constraints

- **In scope (v1 direction)**: Multi-credit tracking, recording usage, viewing remaining balances and history, self-hosted deployment.
- **Out of scope unless a feature spec explicitly expands it**: Payment processing, multi-tenant billing, cloud sync, advanced analytics dashboards.
- **Simplicity**: Prefer boring, proven technology choices documented in `plan.md`; reject scope creep during implementation.
- **Dependencies**: New external services require justification in the plan's Complexity Tracking table.

## Quality Gates

- **Plan gate**: `plan.md` Constitution Check MUST pass before Phase 0 research and again after Phase 1 design.
- **Spec gate**: Each user story MUST be independently testable with Given/When/Then acceptance scenarios.
- **Task gate**: `tasks.md` MUST list test tasks before implementation tasks for each user story.
- **Implementation gate**: No unresolved `NEEDS CLARIFICATION` markers when `/speckit-implement` starts.
- **UX gate**: Primary flows receive a plain-language and accessibility spot-check before the feature is marked complete.

## Governance

- This constitution supersedes ad-hoc implementation choices for the Debt Tracker project.
- Amendments require updating this file, bumping `CONSTITUTION_VERSION` per semantic versioning, and refreshing affected templates.
- **MAJOR**: Principle removal or backward-incompatible redefinition.
- **MINOR**: New principle or materially expanded guidance.
- **PATCH**: Clarifications and non-semantic wording fixes.
- All feature plans, specs, and task lists MUST be reviewed for compliance before implementation.
- Runtime guidance: feature specs under `specs/`, `.cursor/rules/specify-rules.mdc`, and per-feature `quickstart.md`.

**Version**: 1.0.0 | **Ratified**: 2026-05-20 | **Last Amended**: 2026-05-20
