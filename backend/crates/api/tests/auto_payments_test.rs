use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::{Months, Utc};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let db = db::test_support::test_pool().await;
    api::router::app(api::AppState { pool: db.pool }, None)
}

#[tokio::test]
async fn auto_applies_due_regular_payments() {
    let app = test_app().await;
    let start = Utc::now()
        .date_naive()
        .checked_sub_months(Months::new(3))
        .unwrap()
        .to_string();
    let body = serde_json::json!({
        "label": "Auto Loan",
        "setup_mode": "advanced",
        "remaining_balance_minor": 1_000_000,
        "payment_frequency": "monthly",
        "payment_type": "fixed",
        "fixed_payment_minor": 50_000,
        "apr_basis_points": 400,
        "loan_start_date": start
    });
    let create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/loans")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create.status(), StatusCode::OK);
    let created: serde_json::Value =
        serde_json::from_slice(&create.into_body().collect().await.unwrap().to_bytes()).unwrap();
    let id = created["id"].as_str().unwrap();

    let dash = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/dashboard")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let dash_bytes = dash.into_body().collect().await.unwrap().to_bytes();
    let dash_json: serde_json::Value = serde_json::from_slice(&dash_bytes).unwrap();
    let loan = &dash_json["loans"][0];
    assert!(loan["last_payment_date"].as_str().is_some());
    assert!(loan["remaining_balance"]["amount_minor"].as_i64().unwrap() < 1_000_000);

    let payments = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/loans/{id}/payments"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let pay_json: Vec<serde_json::Value> =
        serde_json::from_slice(&payments.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert!(pay_json.len() >= 3);
    assert!(pay_json.iter().all(|p| p["event_type"] == "regular"));

    // Idempotent second dashboard load
    let dash2 = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/dashboard")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let dash2_json: serde_json::Value =
        serde_json::from_slice(&dash2.into_body().collect().await.unwrap().to_bytes()).unwrap();
    assert_eq!(
        dash2_json["loans"][0]["remaining_balance"]["amount_minor"],
        loan["remaining_balance"]["amount_minor"]
    );
}
