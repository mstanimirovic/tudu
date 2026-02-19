use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::{auth::jwt::create_jwt, error::AppError, state::AppState, users::models::CreateUser};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = state
        .users_repo
        .find_by_email(payload.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if user.password_hash != payload.password_hash {
        return Err(AppError::Unauthorized);
    }

    let token = match create_jwt(user.id) {
        Ok(v) => v,
        Err(_) => return Err(AppError::InternalError),
    };

    Ok(Json(AuthResponse {
        token: token,
        user_id: user.id,
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = state
        .users_repo
        .create(CreateUser {
            name: payload.name,
            email: payload.email,
            password_hash: payload.password_hash,
        })
        .await?;

    let token = match create_jwt(user.id) {
        Ok(v) => v,
        Err(_) => return Err(AppError::InternalError),
    };

    Ok(Json(AuthResponse {
        token: token,
        user_id: user.id,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
