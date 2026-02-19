use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub user_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub parent_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCateogry {
    pub parent_id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
}
