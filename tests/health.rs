use serde_json::json;

mod common;
use common::test_server;

#[tokio::test]
async fn health_returns_ok() {
    let server = test_server().await;
    let res = server.get("/health").await;
    res.assert_json(&json!({
        "status": "ok"
    }));
}
