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
async fn dashboard_response_has_required_fields() {
    let app = test_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/dashboard")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("household").is_some());
    assert!(json["household"].get("total_balance").is_some());
    assert!(json["household"].get("total_monthly_obligation").is_some());
    assert!(json.get("loans").is_some());
}
