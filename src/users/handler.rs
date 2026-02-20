use axum::{
    Extension, Router,
    extract::{Json, State},
    middleware::from_fn,
    routing::*,
};

use crate::{
    error::AppError,
    state::AppState,
    users::dto::{UpdateUserRequest, UserDto},
};

pub fn routes() -> Router<AppState> {
    let protected = Router::new()
        .route("/me", get(get_user).patch(update_user).delete(delete_user))
        .layer(from_fn(crate::auth::middleware::auth_middleware));

    Router::new()
        .route("/", get(get_all_users))
        .merge(protected)
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
