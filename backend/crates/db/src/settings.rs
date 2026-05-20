use sqlx::SqlitePool;

pub async fn get_currency(pool: &SqlitePool) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT currency_code FROM settings WHERE id = 1")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn ensure_settings(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR IGNORE INTO settings (id, currency_code, created_at) VALUES (1, 'EUR', datetime('now'))",
    )
    .execute(pool)
    .await?;
    Ok(())
}
