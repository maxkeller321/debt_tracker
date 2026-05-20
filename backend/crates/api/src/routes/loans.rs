use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::Utc;
use domain::interest::compute_interest_summary;
use domain::projection::project_payoff;
use domain::types::{PaymentFrequency, PaymentType};
use serde::Deserialize;

use db::loans_create::CreateLoanParams;

use crate::error::ApiError;
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateLoanBody {
    pub label: String,
    pub setup_mode: String,
    pub remaining_balance_minor: i64,
    pub original_principal_minor: Option<i64>,
    pub payment_frequency: String,
    pub payment_type: String,
    pub fixed_payment_minor: Option<i64>,
    pub apr_basis_points: Option<i32>,
    pub loan_start_date: Option<String>,
    pub recurring_sonderzahlungen: Option<Vec<RecurringInput>>,
    pub backfill_payments: Option<Vec<BackfillInput>>,
}

#[derive(Deserialize)]
pub struct RecurringInput {
    pub amount_minor: i64,
    pub month: u8,
    pub day: u8,
}

#[derive(Deserialize)]
pub struct BackfillInput {
    pub amount_minor: i64,
    pub paid_at: String,
}

#[derive(Deserialize)]
pub struct UpdateLoanBody {
    pub label: Option<String>,
    pub fixed_payment_minor: Option<i64>,
    pub apr_basis_points: Option<i32>,
    pub payment_frequency: Option<String>,
    pub payment_type: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteQuery {
    pub confirm: bool,
}

pub async fn update_loan(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateLoanBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let exists = db::loans::get_loan(&state.pool, &id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    if exists.is_none() {
        return Err(ApiError::not_found("loan not found"));
    }
    let row = exists.unwrap();
    let clear_fixed = body.payment_type.as_deref() == Some("apr");
    let merged_apr = body.apr_basis_points.or(row.apr_basis_points);
    let merged_fixed = if clear_fixed {
        None
    } else {
        body.fixed_payment_minor.or(row.fixed_payment_minor)
    };
    if merged_apr.is_none() {
        return Err(ApiError::bad_request("interest rate (APR) is required"));
    }
    let effective_ptype = body.payment_type.as_deref().unwrap_or(&row.payment_type);
    if effective_ptype == "fixed"
        && (merged_fixed.is_none() || merged_fixed.unwrap_or(0) <= 0)
    {
        return Err(ApiError::bad_request("fixed payment must be positive"));
    }
    db::loans::update_loan(
        &state.pool,
        &id,
        db::loans::UpdateLoanParams {
            label: body.label,
            fixed_payment_minor: body.fixed_payment_minor,
            apr_basis_points: body.apr_basis_points,
            payment_frequency: body.payment_frequency,
            payment_type: body.payment_type,
            notes: body.notes,
            clear_fixed,
        },
    )
    .await
    .map_err(|e| ApiError::internal(e.to_string()))?;
    loan_detail(State(state), Path(id)).await
}

pub async fn create_loan(
    State(state): State<AppState>,
    Json(body): Json<CreateLoanBody>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let freq = parse_frequency(&body.payment_frequency)?;
    let ptype = parse_payment_type(&body.payment_type)?;
    let start = body
        .loan_start_date
        .as_deref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    let recurring: Vec<_> = body
        .recurring_sonderzahlungen
        .unwrap_or_default()
        .into_iter()
        .map(|r| (r.amount_minor, r.month, r.day))
        .collect();
    let backfill: Vec<_> = body
        .backfill_payments
        .unwrap_or_default()
        .into_iter()
        .filter_map(|b| {
            chrono::NaiveDate::parse_from_str(&b.paid_at, "%Y-%m-%d")
                .ok()
                .map(|d| (b.amount_minor, d))
        })
        .collect();
    let id = db::loans_create::create_loan(
        &state.pool,
        CreateLoanParams {
            label: body.label,
            setup_mode: body.setup_mode,
            remaining_balance_minor: body.remaining_balance_minor,
            original_principal_minor: body.original_principal_minor,
            payment_frequency: freq,
            payment_type: ptype,
            fixed_payment_minor: body.fixed_payment_minor,
            apr_basis_points: body.apr_basis_points,
            loan_start_date: start,
            recurring,
            backfill,
        },
    )
    .await
    .map_err(ApiError::bad_request)?;
    loan_detail(State(state), Path(id)).await
}

pub async fn loan_detail(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let as_of = Utc::now().date_naive();
    db::payment_events::apply_due_regular_payments(&state.pool, &id, as_of)
        .await
        .map_err(ApiError::bad_request)?;
    let row = db::loans::get_loan(&state.pool, &id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("loan not found"))?;
    let calc = db::loans::load_loan_calc(&state.pool, &row)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    let currency = db::get_currency(&state.pool).await.map_err(|e| ApiError::internal(e.to_string()))?;
    let projection = project_payoff(&calc, as_of);
    let interest = compute_interest_summary(&calc, as_of);
    let pending = db::scheduled_sonderzahlungen::list_pending(&state.pool, &id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(Json(serde_json::json!({
        "id": calc.id,
        "label": calc.label,
        "remaining_balance": { "amount_minor": calc.remaining_balance_minor, "currency": currency },
        "periodic_payment": { "amount_minor": projection.periodic_payment_minor, "currency": currency },
        "payment_frequency": row.payment_frequency,
        "last_payment_date": calc
            .payments
            .iter()
            .filter(|p| p.event_type == "regular")
            .map(|p| p.paid_at)
            .max()
            .map(|d| d.to_string()),
        "projected_payoff_date": projection.projected_payoff_date.map(|d| d.to_string()),
        "payment_type": row.payment_type,
        "apr_percent": calc.apr_basis_points.map(|b| b as f64 / 100.0),
        "interest_paid_to_date": { "amount_minor": interest.interest_paid_minor, "currency": currency },
        "interest_remaining_estimate": { "amount_minor": interest.interest_remaining_minor, "currency": currency },
        "interest_message": interest.message,
        "upcoming_scheduled": pending,
        "progress_percent": domain::dashboard::build_dashboard(std::slice::from_ref(&calc), &currency, Utc::now().date_naive()).loans.first().map(|l| l.progress_percent).unwrap_or(0.0),
    })))
}

pub async fn delete_loan(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(q): Query<DeleteQuery>,
) -> Result<axum::http::StatusCode, ApiError> {
    if !q.confirm {
        return Err(ApiError::bad_request("confirm=true required"));
    }
    db::loans::delete_loan(&state.pool, &id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn archive_loan(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    db::loans::archive_loan(&state.pool, &id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    loan_detail(State(state), Path(id)).await
}

fn parse_frequency(s: &str) -> Result<PaymentFrequency, ApiError> {
    match s {
        "monthly" => Ok(PaymentFrequency::Monthly),
        "yearly" => Ok(PaymentFrequency::Yearly),
        _ => Err(ApiError::bad_request("invalid payment_frequency")),
    }
}

fn parse_payment_type(s: &str) -> Result<PaymentType, ApiError> {
    match s {
        "fixed" => Ok(PaymentType::Fixed),
        "apr" => Ok(PaymentType::Apr),
        _ => Err(ApiError::bad_request("invalid payment_type")),
    }
}
