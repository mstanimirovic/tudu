use axum::{
    Extension,
    extract::{Json, State},
};

use crate::{
    error::AppError,
    state::AppState,
    users::models::{UpdateUser, User},
};

pub async fn get_all_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = state.users_repo.find_all().await?;
    Ok(Json(users))
}

pub async fn get_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<Json<User>, AppError> {
    let user = state.users_repo.find_by_id(user_id).await?;
    match user {
        Some(v) => Ok(Json(v)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    let user = state.users_repo.update(user_id, payload).await?;
    match user {
        Some(v) => Ok(Json(v)),
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
