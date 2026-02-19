use hyper::StatusCode;
use serde_json::{Value, json};
mod common;

use crate::common::test_server;

#[tokio::test]
async fn users_returns_empty_list() {
    let server = test_server().await;
    let res = server.get("/api/users").await;
    res.assert_json(&json!([]));
}

#[tokio::test]
async fn should_register() {
    let server = test_server().await;
    let res = server
        .post("/auth/register")
        .json(&json!(
            {
                "name": "mladen",
                "email": "ms@email.com",
                "password_hash": "huha"
            }
        ))
        .await;

    res.assert_status(StatusCode::OK);

    let body: Value = res.json();
    assert!(body.is_object());
    assert!(body.get("token").is_some());
    assert!(body.get("user_id").is_some());
}

#[tokio::test]
async fn should_login() {
    let server = test_server().await;
    server
        .post("/auth/register")
        .json(&json!(
            {
                "name": "mladen",
                "email": "ms@email.com",
                "password_hash": "huha"
            }
        ))
        .await;

    let res = server
        .post("/auth/login")
        .json(&json!(
            {
                "email": "ms@email.com",
                "password_hash": "huha"
            }
        ))
        .await;

    res.assert_status(StatusCode::OK);

    let body: Value = res.json();
    assert!(body.is_object());
    assert!(body.get("token").is_some());
    assert!(body.get("user_id").is_some());
}
