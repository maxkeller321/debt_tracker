use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn test_app() -> axum::Router {
    let db = db::test_support::test_pool().await;
    let state = api::AppState { pool: db.pool };
    api::router::app(state, None)
}

#[tokio::test]
async fn create_loan_quick() {
    let app = test_app().await;
    let body = serde_json::json!({
        "label": "Mortgage",
        "setup_mode": "quick",
        "remaining_balance_minor": 20000000,
        "payment_frequency": "monthly",
        "payment_type": "fixed",
        "fixed_payment_minor": 120000,
        "apr_basis_points": 375
    });
    let response = app
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
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8_lossy(&bytes);
    assert_eq!(status, StatusCode::OK, "body: {text}");
}
