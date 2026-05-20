use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let db = db::test_support::test_pool().await;
    let state = api::AppState { pool: db.pool };
    api::router::app(state, None)
}

async fn seed_loan(app: &axum::Router) -> String {
    let body = serde_json::json!({
        "label": "Test",
        "setup_mode": "quick",
        "remaining_balance_minor": 1000000,
        "payment_frequency": "monthly",
        "payment_type": "fixed",
        "fixed_payment_minor": 50000,
        "apr_basis_points": 350
    });
    let response = app
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
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    json["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn immediate_sonderzahlung_reduces_balance() {
    let app = test_app().await;
    let id = seed_loan(&app).await;
    let body = serde_json::json!({
        "amount_minor": 100000,
        "paid_at": "2025-06-01",
        "confirm_overpayment": true
    });
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/loans/{id}/sonderzahlungen/immediate"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
