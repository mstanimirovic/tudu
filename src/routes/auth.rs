use axum::{Json, Router, extract::State, routing::post};

use crate::{
    error::AppError,
    models::{
        auth::{AuthResponse, LoginRequest, RegisterRequest},
        user::{User, UserPublic},
    },
    state::AppState,
    util::jwt::create_jwt,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user: User = state
        .users_repo
        .find_by_email(payload.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if user.password != payload.password {
        return Err(AppError::Unauthorized);
    }

    let token = match create_jwt(user.id) {
        Ok(v) => v,
        Err(_) => return Err(AppError::InternalError),
    };

    Ok(Json(AuthResponse {
        token: token,
        user: UserPublic::from(user),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = state.users_repo.create(payload).await?;

    let token = match create_jwt(user.id) {
        Ok(v) => v,
        Err(_) => return Err(AppError::InternalError),
    };

    Ok(Json(AuthResponse {
        token: token,
        user: UserPublic::from(user),
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
