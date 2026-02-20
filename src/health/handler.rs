use super::dto::HealthResponse;
use crate::{error::AppError, state::AppState};
use axum::{Json, Router, routing::get};

pub async fn handler() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse::ok()))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(handler))
}
