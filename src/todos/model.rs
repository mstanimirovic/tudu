use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub category_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub priority: i32,
    pub due_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
