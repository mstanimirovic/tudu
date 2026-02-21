use axum_test::TestServer;
use chrono::DateTime;
use hyper::StatusCode;
use serde_json::json;
use tudu::todos::dto::TodoDto;
use tudu::{auth::dto::AuthResponse, todos::dto::CreateTodoRequest};
mod common;

use crate::common::test_server;

async fn create_user(server: &TestServer) -> String {
    let res = server
        .post("/auth/register")
        .json(&json!(
            {
                "name": "mladen",
                "email": "ms@email.com",
                "password": "huha"
            }
        ))
        .await;

    res.assert_status(StatusCode::OK);

    let body = res.json::<AuthResponse>();
    return body.token;
}

#[tokio::test]
async fn should_create_todo() {
    let server = test_server().await;
    let token = create_user(&server).await;
    let res = server
        .post("/api/todos")
        .authorization_bearer(token)
        .json(&json!(CreateTodoRequest {
            category_id: None,
            title: "Study".to_string(),
            description: None,
            priority: 1,
            due_at: DateTime::default()
        }))
        .await;

    res.assert_status(StatusCode::OK);
    let todo = res.json::<TodoDto>();

    assert_eq!(todo.title, "Study");
    assert_eq!(todo.category_id, None);
}

#[tokio::test]
async fn should_get_todo() {
    let server = test_server().await;
    let token = create_user(&server).await;
    let res = server
        .post("/api/todos")
        .authorization_bearer(token.clone())
        .json(&json!(CreateTodoRequest {
            category_id: None,
            title: "Study".to_string(),
            description: None,
            priority: 1,
            due_at: DateTime::default()
        }))
        .await;

    res.assert_status(StatusCode::OK);
    let id = res.json::<TodoDto>().id;

    let get_res = server
        .get(format!("/api/todos/{}", id).as_str())
        .authorization_bearer(token)
        .await;

    get_res.assert_status(StatusCode::OK);
    assert_eq!(
        res.json::<TodoDto>().created_at,
        get_res.json::<TodoDto>().created_at
    );
}
