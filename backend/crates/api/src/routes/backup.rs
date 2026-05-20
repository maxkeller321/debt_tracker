use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;

use crate::error::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct ImportQuery {
    pub confirm: bool,
}

pub async fn export_data(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    let bundle = db::export::export_all(&state.pool)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(Json(serde_json::to_value(bundle).unwrap()))
}

pub async fn import_data(
    State(state): State<AppState>,
    Query(q): Query<ImportQuery>,
    Json(body): Json<db::import::ImportBundle>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if !q.confirm {
        return Err(ApiError::bad_request("confirm=true required"));
    }
    db::import::import_replace(&state.pool, body)
        .await
        .map_err(ApiError::bad_request)?;
    Ok(Json(serde_json::json!({ "ok": true })))
}
