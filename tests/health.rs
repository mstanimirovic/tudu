use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use tower::ServiceExt; // oneshot

use tudu::{build_app, build_state};
mod common;
use common::test_pool;

#[tokio::test]
async fn health_returns_ok() {
    let pool = test_pool().await;
    let state = build_state(pool).await;
    let app = build_app(state);

    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), 200);

    let body = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(v["status"], "ok");
}
