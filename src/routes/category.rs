use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    middleware,
    routing::get,
};

use crate::{
    error::AppError,
    models::category::{Category, CreateCategoryRequest, UpdateCateogryRequest},
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
            crate::middleware::auth::auth_middleware,
        ))
}

pub async fn create_category(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<Category>, AppError> {
    let category = state.categories_repo.create(user_id, payload).await?;
    Ok(Json(category))
}

pub async fn get_categories(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = state.categories_repo.find_all_by_user(user_id).await?;
    Ok(Json(categories))
}

pub async fn get_category_by_id(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Category>, AppError> {
    let category = match state.categories_repo.find_by_id(id).await? {
        Some(v) => v,
        None => return Err(AppError::NotFound),
    };
    if let Some(v) = category.user_id {
        if v != user_id {
            return Err(AppError::Forbidden);
        }
    }
    Ok(Json(category))
}

pub async fn update_category(
    Extension(user_id): Extension<i64>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateCateogryRequest>,
) -> Result<Json<Category>, AppError> {
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

    match state.categories_repo.update(id, payload).await? {
        Some(v) => Ok(Json(v)),
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
