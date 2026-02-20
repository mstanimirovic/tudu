use axum::{
    Extension, Router,
    extract::{Json, State},
    routing::*,
};

use crate::{
    error::AppError,
    state::AppState,
    users::dto::{UpdateUserRequest, UserDto},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_user).patch(update_user).delete(delete_user))
        .route("/", get(get_all_users))
}

pub async fn get_all_users(State(state): State<AppState>) -> Result<Json<Vec<UserDto>>, AppError> {
    Ok(Json(
        state
            .users_repo
            .find_all()
            .await?
            .iter()
            .map(UserDto::from)
            .collect(),
    ))
}

pub async fn get_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<Json<UserDto>, AppError> {
    match state.users_repo.find_by_id(user_id).await? {
        Some(v) => Ok(Json(UserDto::from(v))),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserDto>, AppError> {
    match state.users_repo.update(user_id, payload.into()).await? {
        Some(v) => Ok(Json(UserDto::from(v))),
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
