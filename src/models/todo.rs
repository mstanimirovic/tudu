use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub priority: i64,
    pub due_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub category_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub priority: i64,
    pub due_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub category_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub done: Option<bool>,
    pub priority: Option<i64>,
    pub due_at: Option<i64>,
}
