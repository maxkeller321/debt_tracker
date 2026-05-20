use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn schedule(
    pool: &SqlitePool,
    loan_id: &str,
    amount_minor: i64,
    due_date: chrono::NaiveDate,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO scheduled_sonderzahlungen (id, loan_id, amount_minor, due_date, status, created_at) VALUES (?, ?, ?, ?, 'pending', ?)",
    )
    .bind(&id)
    .bind(loan_id)
    .bind(amount_minor)
    .bind(due_date.to_string())
    .bind(&now)
    .execute(pool)
    .await?;
    Ok(id)
}

pub async fn cancel(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE scheduled_sonderzahlungen SET status = 'cancelled' WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_pending(pool: &SqlitePool, loan_id: &str) -> Result<Vec<serde_json::Value>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, i64, String, String)>(
        "SELECT id, amount_minor, due_date, status FROM scheduled_sonderzahlungen WHERE loan_id = ? AND status = 'pending' ORDER BY due_date",
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|(id, amount_minor, due_date, status)| {
            serde_json::json!({
                "id": id,
                "amount_minor": amount_minor,
                "due_date": due_date,
                "status": status,
            })
        })
        .collect())
}
