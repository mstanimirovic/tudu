#[derive(Debug, sqlx::FromRow)]
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
