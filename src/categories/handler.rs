use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    routing::get,
};

use crate::{
    categories::dto::{CategoryDto, CreateCategoryRequest, UpdateCategoryRequest},
    error::AppError,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/{id}",
            get(get_category_by_id)
                .patch(update_category)
                .delete(delete_category),
        )
        .layer(middleware::from_fn(
            crate::auth::middleware::auth_middleware,
        ))
}

pub async fn create_category(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<CategoryDto>, AppError> {
    let category = state
        .categories_repo
        .create(user_id, payload.into())
        .await?;
    Ok(Json(CategoryDto::from(category)))
}

pub async fn get_categories(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryDto>>, AppError> {
    Ok(Json(
        state
            .categories_repo
            .find_all_by_user(user_id)
            .await?
            .iter()
            .map(CategoryDto::from)
            .collect(),
    ))
}

pub async fn get_category_by_id(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<CategoryDto>, AppError> {
    let category = match state.categories_repo.find_by_id(id).await? {
        Some(v) => v,
        None => return Err(AppError::NotFound),
    };
    if let Some(v) = category.user_id {
        if v != user_id {
            return Err(AppError::Forbidden);
        }
    }
    Ok(Json(CategoryDto::from(category)))
}

pub async fn update_category(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<CategoryDto>, AppError> {
    match state.categories_repo.find_by_id(id).await? {
        Some(v) => {
            if let Some(cuser) = v.user_id
                && cuser != user_id
            {
                return Err(AppError::Forbidden);
            }
        }
        None => return Err(AppError::NotFound),
    };

    match state.categories_repo.update(id, payload.into()).await? {
        Some(v) => Ok(Json(CategoryDto::from(v))),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_category(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let category = match state.categories_repo.find_by_id(id).await? {
        Some(v) => v,
        None => return Err(AppError::NotFound),
    };

    if let Some(cat_user_id) = category.user_id {
        if cat_user_id != user_id {
            return Err(AppError::Forbidden);
        }
    }

    let ra = state.categories_repo.delete(id).await?;
    if ra != 1 {
        return Err(AppError::InternalError);
    }

    Ok(())
}
