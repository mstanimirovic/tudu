use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: String,
}

impl HealthResponse {
    pub fn ok() -> Self {
        Self {
            status: "ok".to_string(),
        }
    }
}
