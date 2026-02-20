use axum::{Json, Router, routing::get};

use crate::{error::AppError, models::health::HealthResponse, state::AppState};

pub async fn handler() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse::ok()))
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(handler))
}
