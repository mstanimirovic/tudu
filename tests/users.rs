use hyper::StatusCode;
use serde_json::json;
use tudu::models::{auth::AuthResponse, user::UserPublic};
mod common;

use crate::common::test_server;

#[tokio::test]
async fn should_get_user_profile() {
    let server = test_server().await;
    let reg_res = server
        .post("/auth/register")
        .json(&json!(
            {
                "name": "mladen",
                "email": "ms@email.com",
                "password": "huha"
            }
        ))
        .await;

    reg_res.assert_status(StatusCode::OK);

    let token = reg_res.json::<AuthResponse>().token;
    let res = server
        .get("/api/users/me")
        .authorization_bearer(token)
        .await;

    res.assert_status(StatusCode::OK);
    let user = res.json::<UserPublic>();

    assert_eq!(user.name, "mladen");
    assert_eq!(user.email, "ms@email.com");
}
