use chrono::Utc;
use domain::amortization::effective_periodic_payment;
use domain::due_payments::due_regular_payment_dates;
use domain::payment_split::split_payment;
use domain::types::PaymentFrequency;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::loans::{get_loan, load_loan_calc, update_balance};

pub async fn last_regular_payment_date(
    pool: &SqlitePool,
    loan_id: &str,
) -> Result<Option<chrono::NaiveDate>, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT MAX(paid_at) FROM payment_events WHERE loan_id = ? AND event_type = 'regular'",
    )
    .bind(loan_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|(d,)| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()))
}

async fn has_regular_on_date(
    pool: &SqlitePool,
    loan_id: &str,
    paid_at: chrono::NaiveDate,
) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM payment_events WHERE loan_id = ? AND event_type = 'regular' AND paid_at = ? LIMIT 1",
    )
    .bind(loan_id)
    .bind(paid_at.to_string())
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Apply all due regular installments through `as_of` (idempotent per due date).
pub async fn apply_due_regular_payments(
    pool: &SqlitePool,
    loan_id: &str,
    as_of: chrono::NaiveDate,
) -> Result<u32, String> {
    let row = get_loan(pool, loan_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "loan not found".to_string())?;
    if row.status != "active" || row.remaining_balance_minor <= 0 {
        return Ok(0);
    }

    let frequency = if row.payment_frequency == "yearly" {
        PaymentFrequency::Yearly
    } else {
        PaymentFrequency::Monthly
    };

    let start = row
        .loan_start_date
        .as_deref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .or_else(|| chrono::NaiveDate::parse_from_str(&row.created_at[..10], "%Y-%m-%d").ok())
        .unwrap_or(as_of);

    let last_regular = last_regular_payment_date(pool, loan_id)
        .await
        .map_err(|e| e.to_string())?;

    let due_dates = due_regular_payment_dates(start, frequency, last_regular, as_of);
    let mut applied = 0u32;

    for due in due_dates {
        if has_regular_on_date(pool, loan_id, due)
            .await
            .map_err(|e| e.to_string())?
        {
            continue;
        }
        let fresh = get_loan(pool, loan_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "loan not found".to_string())?;
        if fresh.remaining_balance_minor <= 0 {
            break;
        }
        let calc = load_loan_calc(pool, &fresh)
            .await
            .map_err(|e| e.to_string())?;
        let amount = effective_periodic_payment(
            calc.remaining_balance_minor,
            calc.apr_basis_points,
            calc.fixed_payment_minor,
            calc.payment_frequency,
        );
        if amount <= 0 {
            break;
        }
        record_regular_payment(pool, loan_id, amount, due, Some("auto".into())).await?;
        applied += 1;
    }

    Ok(applied)
}

pub async fn sync_all_due_regular_payments(
    pool: &SqlitePool,
    as_of: chrono::NaiveDate,
) -> Result<(), String> {
    let rows = crate::loans::list_loans(pool, false)
        .await
        .map_err(|e| e.to_string())?;
    for row in rows {
        if row.status == "active" {
            apply_due_regular_payments(pool, &row.id, as_of).await?;
        }
    }
    Ok(())
}

pub async fn record_regular_payment(
    pool: &SqlitePool,
    loan_id: &str,
    amount_minor: i64,
    paid_at: chrono::NaiveDate,
    note: Option<String>,
) -> Result<(), String> {
    let row = get_loan(pool, loan_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "loan not found".to_string())?;
    let calc = load_loan_calc(pool, &row).await.map_err(|e| e.to_string())?;
    let split = split_payment(&calc, amount_minor, calc.remaining_balance_minor);
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        r#"INSERT INTO payment_events (
            id, loan_id, event_type, amount_minor, interest_portion_minor,
            principal_portion_minor, balance_after_minor, paid_at, note, created_at
        ) VALUES (?, ?, 'regular', ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(loan_id)
    .bind(amount_minor)
    .bind(split.interest_portion_minor)
    .bind(split.principal_portion_minor)
    .bind(split.balance_after_minor)
    .bind(paid_at.to_string())
    .bind(note)
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;
    update_balance(pool, loan_id, split.balance_after_minor)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn record_sonderzahlung(
    pool: &SqlitePool,
    loan_id: &str,
    amount_minor: i64,
    paid_at: chrono::NaiveDate,
) -> Result<(), String> {
    let row = get_loan(pool, loan_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "loan not found".to_string())?;
    let calc = load_loan_calc(pool, &row).await.map_err(|e| e.to_string())?;
    let balance_after = (calc.remaining_balance_minor - amount_minor).max(0);
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let interest = 0i64;
    let principal = amount_minor;
    sqlx::query(
        r#"INSERT INTO payment_events (
            id, loan_id, event_type, amount_minor, interest_portion_minor,
            principal_portion_minor, balance_after_minor, paid_at, created_at
        ) VALUES (?, ?, 'sonderzahlung', ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(loan_id)
    .bind(amount_minor)
    .bind(interest)
    .bind(principal)
    .bind(balance_after)
    .bind(paid_at.to_string())
    .bind(&now)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;
    update_balance(pool, loan_id, balance_after)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn list_payments(pool: &SqlitePool, loan_id: &str) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String, i64, i64, i64, i64, String)>(
        r#"SELECT id, event_type, amount_minor, interest_portion_minor,
           principal_portion_minor, balance_after_minor, paid_at
           FROM payment_events WHERE loan_id = ? ORDER BY paid_at DESC"#,
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(id, event_type, amount_minor, i, p, b, paid_at)| {
            serde_json::json!({
                "id": id,
                "event_type": event_type,
                "amount_minor": amount_minor,
                "interest_portion_minor": i,
                "principal_portion_minor": p,
                "balance_after_minor": b,
                "paid_at": paid_at,
            })
        })
        .collect())
}
