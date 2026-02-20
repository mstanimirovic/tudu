use axum::{
    Extension, Router,
    extract::{Json, Path, Query, State},
    routing::*,
};

use crate::{
    error::AppError,
    state::AppState,
    todos::{
        dto::{CreateTodoRequest, TodoDto, UpdateTodoRequest},
        filter::TodosFilter,
        query::TodosQuery,
    },
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_todo).get(list_todos))
        .route(
            "/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
}

pub async fn create_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<TodoDto>, AppError> {
    let todo = state.todos_repo.create(user_id, payload.into()).await?;
    if todo.user_id != user_id {
        return Err(AppError::Forbidden);
    }
    Ok(Json(TodoDto::from(todo)))
}

pub async fn list_todos(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Query(query): Query<TodosQuery>,
) -> Result<Json<Vec<TodoDto>>, AppError> {
    let filter = TodosFilter::from_query(query);
    let rows = state.todos_repo.find_many(user_id, filter).await?;
    let dtos = rows.iter().map(TodoDto::from).collect();
    Ok(Json(dtos))
}

pub async fn get_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<TodoDto>, AppError> {
    let todo = state.todos_repo.find_by_id(id).await?;
    match todo {
        Some(v) => {
            if v.user_id != user_id {
                return Err(AppError::Forbidden);
            }
            Ok(Json(TodoDto::from(v)))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn update_todo(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<TodoDto>, AppError> {
    match state.todos_repo.find_by_id(id).await? {
        Some(v) => {
            if v.user_id != user_id {
                return Err(AppError::Forbidden);
            }
        }
        None => return Err(AppError::NotFound),
    };

    match state.todos_repo.update(id, payload.into()).await? {
        Some(v) => Ok(Json(TodoDto::from(v))),
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
