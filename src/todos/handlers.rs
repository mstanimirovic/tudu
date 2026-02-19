use axum::{
    Extension,
    extract::{Json, Path, State},
};
use sqlx::SqlitePool;

use crate::{
    error::AppError,
    state::AppState,
    todos::models::{CreateTodo, Todo, UpdateTodo},
};

pub async fn create_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let todo = state.todos_repo.create(user_id, payload).await?;
    if todo.user_id != user_id {
        return Err(AppError::Forbidden);
    }
    Ok(Json(todo))
}

pub async fn list_todos(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = state.todos_repo.find_all_by_user(user_id).await?;
    Ok(Json(todos))
}

pub async fn get_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Todo>, AppError> {
    let todo = state.todos_repo.find_by_id(id).await?;
    match todo {
        Some(v) => {
            if v.user_id != user_id {
                return Err(AppError::Forbidden);
            }
            Ok(Json(v))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn update_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let todo = match state.todos_repo.find_by_id(id).await? {
        Some(v) => v,
        None => return Err(AppError::Forbidden),
    };

    match state.todos_repo.update(id, payload).await? {
        Some(v) => Ok(Json(v)),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let todo = match state.todos_repo.find_by_id(id).await? {
        Some(v) => v,
        None => return Err(AppError::NotFound),
    };

    if todo.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    let ra = state.todos_repo.delete(id).await?;
    if ra != 1 {
        return Err(AppError::InternalError);
    }

    Ok(())
}
