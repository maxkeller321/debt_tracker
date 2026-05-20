CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    currency_code TEXT NOT NULL DEFAULT 'EUR',
    created_at TEXT NOT NULL
);

INSERT OR IGNORE INTO settings (id, currency_code, created_at)
VALUES (1, 'EUR', datetime('now'));

CREATE TABLE IF NOT EXISTS loans (
    id TEXT PRIMARY KEY,
    label TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    setup_mode TEXT NOT NULL,
    original_principal_minor INTEGER,
    remaining_balance_minor INTEGER NOT NULL,
    payment_frequency TEXT NOT NULL,
    payment_type TEXT NOT NULL,
    fixed_payment_minor INTEGER,
    apr_basis_points INTEGER,
    loan_start_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT,
    notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_loans_status ON loans(status);

CREATE TABLE IF NOT EXISTS recurring_sonderzahlungen (
    id TEXT PRIMARY KEY,
    loan_id TEXT NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    amount_minor INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS scheduled_sonderzahlungen (
    id TEXT PRIMARY KEY,
    loan_id TEXT NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    amount_minor INTEGER NOT NULL,
    due_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    executed_payment_id TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_scheduled_loan_status ON scheduled_sonderzahlungen(loan_id, status, due_date);

CREATE TABLE IF NOT EXISTS payment_events (
    id TEXT PRIMARY KEY,
    loan_id TEXT NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    amount_minor INTEGER NOT NULL,
    interest_portion_minor INTEGER NOT NULL,
    principal_portion_minor INTEGER NOT NULL,
    balance_after_minor INTEGER NOT NULL,
    paid_at TEXT NOT NULL,
    note TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_payment_events_loan_paid ON payment_events(loan_id, paid_at);
