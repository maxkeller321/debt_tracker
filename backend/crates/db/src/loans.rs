use chrono::NaiveDate;
use domain::types::{
    LoanCalcInput, LoanStatus, PaymentFrequency, PaymentRecord, PaymentType, RecurringExtra,
    ScheduledExtra,
};
use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct LoanRow {
    pub id: String,
    pub label: String,
    pub status: String,
    pub setup_mode: String,
    pub original_principal_minor: Option<i64>,
    pub remaining_balance_minor: i64,
    pub payment_frequency: String,
    pub payment_type: String,
    pub fixed_payment_minor: Option<i64>,
    pub apr_basis_points: Option<i32>,
    pub loan_start_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub notes: Option<String>,
}

pub async fn list_loans(pool: &SqlitePool, include_archived: bool) -> Result<Vec<LoanRow>, sqlx::Error> {
    let rows = if include_archived {
        sqlx::query_as::<_, LoanRow>(
            "SELECT id, label, status, setup_mode, original_principal_minor, remaining_balance_minor,
             payment_frequency, payment_type, fixed_payment_minor, apr_basis_points, loan_start_date,
             created_at, updated_at, archived_at, notes FROM loans ORDER BY created_at DESC",
        )
            .fetch_all(pool)
            .await?
    } else {
        sqlx::query_as::<_, LoanRow>(
            "SELECT id, label, status, setup_mode, original_principal_minor, remaining_balance_minor,
             payment_frequency, payment_type, fixed_payment_minor, apr_basis_points, loan_start_date,
             created_at, updated_at, archived_at, notes FROM loans WHERE status = 'active' ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}

pub async fn get_loan(pool: &SqlitePool, id: &str) -> Result<Option<LoanRow>, sqlx::Error> {
    sqlx::query_as::<_, LoanRow>(
        "SELECT id, label, status, setup_mode, original_principal_minor, remaining_balance_minor,
         payment_frequency, payment_type, fixed_payment_minor, apr_basis_points, loan_start_date,
         created_at, updated_at, archived_at, notes FROM loans WHERE id = ?",
    )
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn load_loan_calc(pool: &SqlitePool, row: &LoanRow) -> Result<LoanCalcInput, sqlx::Error> {
    let recurring = sqlx::query_as::<_, (i64, u8, u8, i64)>(
        "SELECT amount_minor, month, day, enabled FROM recurring_sonderzahlungen WHERE loan_id = ?",
    )
    .bind(&row.id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(amount_minor, month, day, enabled)| RecurringExtra {
        amount_minor,
        month,
        day,
        enabled: enabled != 0,
    })
    .collect();

    let scheduled_rows = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT amount_minor, due_date, status FROM scheduled_sonderzahlungen WHERE loan_id = ?",
    )
    .bind(&row.id)
    .fetch_all(pool)
    .await?;

    let mut scheduled = Vec::new();
    for (amount_minor, due_date, status) in scheduled_rows {
        if let Ok(d) = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d") {
            scheduled.push(ScheduledExtra {
                amount_minor,
                due_date: d,
                status,
            });
        }
    }

    let payments_rows = sqlx::query_as::<_, (String, String, i64, i64, i64)>(
        "SELECT paid_at, event_type, amount_minor, interest_portion_minor, principal_portion_minor FROM payment_events WHERE loan_id = ? ORDER BY paid_at",
    )
    .bind(&row.id)
    .fetch_all(pool)
    .await?;

    let payments: Vec<PaymentRecord> = payments_rows
        .into_iter()
        .filter_map(
            |(paid_at, event_type, amount_minor, interest_portion_minor, principal_portion_minor)| {
                NaiveDate::parse_from_str(&paid_at, "%Y-%m-%d").ok().map(|d| PaymentRecord {
                    paid_at: d,
                    amount_minor,
                    interest_portion_minor,
                    principal_portion_minor,
                    event_type,
                })
            },
        )
        .collect();

    let start = row
        .loan_start_date
        .as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .or_else(|| {
            NaiveDate::parse_from_str(&row.created_at[..10], "%Y-%m-%d").ok()
        })
        .unwrap_or_else(|| chrono::Utc::now().date_naive());

    Ok(LoanCalcInput {
        id: row.id.clone(),
        label: row.label.clone(),
        status: row.status.parse().unwrap_or(LoanStatus::Active),
        remaining_balance_minor: row.remaining_balance_minor,
        original_principal_minor: row.original_principal_minor,
        payment_frequency: if row.payment_frequency == "yearly" {
            PaymentFrequency::Yearly
        } else {
            PaymentFrequency::Monthly
        },
        payment_type: if row.payment_type == "apr" {
            PaymentType::Apr
        } else {
            PaymentType::Fixed
        },
        fixed_payment_minor: row.fixed_payment_minor,
        apr_basis_points: row.apr_basis_points,
        loan_start_date: start,
        recurring_extras: recurring,
        scheduled_extras: scheduled,
        payments,
    })
}

pub async fn load_active_calc_inputs(pool: &SqlitePool) -> Result<Vec<LoanCalcInput>, sqlx::Error> {
    let rows = list_loans(pool, false).await?;
    let mut out = Vec::new();
    for row in rows {
        out.push(load_loan_calc(pool, &row).await?);
    }
    Ok(out)
}

pub async fn update_balance(
    pool: &SqlitePool,
    loan_id: &str,
    balance_minor: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE loans SET remaining_balance_minor = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(balance_minor)
    .bind(loan_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_loan(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM loans WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub struct UpdateLoanParams {
    pub label: Option<String>,
    pub fixed_payment_minor: Option<i64>,
    pub apr_basis_points: Option<i32>,
    pub payment_frequency: Option<String>,
    pub payment_type: Option<String>,
    pub notes: Option<String>,
    pub clear_fixed: bool,
}

pub async fn update_loan(
    pool: &SqlitePool,
    id: &str,
    params: UpdateLoanParams,
) -> Result<(), sqlx::Error> {
    let row = get_loan(pool, id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    let label = params.label.unwrap_or(row.label);
    let freq = params.payment_frequency.unwrap_or(row.payment_frequency);
    let notes = params.notes.or(row.notes);
    let fixed = if params.clear_fixed {
        None
    } else {
        params.fixed_payment_minor.or(row.fixed_payment_minor)
    };
    let apr = params.apr_basis_points.or(row.apr_basis_points);
    let ptype = params
        .payment_type
        .as_deref()
        .unwrap_or_else(|| {
            if fixed.is_some() {
                "fixed"
            } else {
                &row.payment_type
            }
        });
    sqlx::query(
        r#"UPDATE loans SET label = ?, payment_frequency = ?, payment_type = ?,
           fixed_payment_minor = ?, apr_basis_points = ?, notes = ?, updated_at = datetime('now')
           WHERE id = ?"#,
    )
    .bind(&label)
    .bind(&freq)
    .bind(ptype)
    .bind(fixed)
    .bind(apr)
    .bind(&notes)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn archive_loan(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE loans SET status = 'archived', archived_at = datetime('now'), updated_at = datetime('now') WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
