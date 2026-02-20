use axum::{
    Extension, Router,
    extract::{Json, State},
    middleware::from_fn,
    routing::*,
};

use crate::{
    error::AppError,
    models::user::{UpdateUserRequest, UserPublic},
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    let protected = Router::new()
        .route("/me", get(get_user).patch(update_user).delete(delete_user))
        .layer(from_fn(crate::middleware::auth::auth_middleware));

    Router::new()
        .route("/", get(get_all_users))
        .merge(protected)
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserPublic>>, AppError> {
    let users = state
        .users_repo
        .find_all()
        .await?
        .iter()
        .map(UserPublic::from)
        .collect();

    Ok(Json(users))
}

pub async fn get_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<Json<UserPublic>, AppError> {
    let user = state.users_repo.find_by_id(user_id).await?;
    match user {
        Some(v) => Ok(Json(UserPublic::from(v))),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserPublic>, AppError> {
    let user = state.users_repo.update(user_id, payload).await?;
    match user {
        Some(v) => Ok(Json(UserPublic::from(v))),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    state.users_repo.delete(user_id).await?;
    Ok(())
}
