use axum::{
    Extension,
    extract::{Json, Path, State},
};
use sqlx::SqlitePool;

use crate::{
    error::AppError,
    state::AppState,
    users::models::{CreateUser, UpdateUser, User},
};

// pub async fn create_user(
//     State(state): State<AppState>,
//     Json(payload): Json<CreateUser>,
// ) -> Result<Json<User>, AppError> {
//     let user = state.users_repo.create(payload).await?;
//     Ok(Json(user))
// }

pub async fn get_all_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = state.users_repo.find_all().await?;
    Ok(Json(users))
}

pub async fn get_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    if user_id != id {
        return Err(AppError::Forbidden);
    }
    let user = state.users_repo.find_by_id(id).await?;
    match (user) {
        Some(v) => Ok(Json(v)),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    if user_id != id {
        return Err(AppError::Forbidden);
    }
    let user = state.users_repo.update(id, payload).await?;
    match (user) {
        Some(v) => Ok(Json(v)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_user(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    if user_id != id {
        return Err(AppError::Forbidden);
    }
    state.users_repo.delete(id).await?;
    Ok(())
}
