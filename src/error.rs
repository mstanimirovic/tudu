use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    Unauthorized,
    Forbidden,
    InternalError,
    Conflict,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized - Invalid or missing token",
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Access denied - Insufficient permissions",
            ),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::Conflict => (StatusCode::CONFLICT, "Conflict with data"),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}
