use axum::Json;
use serde::Serialize;

use crate::error::AppError;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
}

pub async fn handler() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
    }))
}
