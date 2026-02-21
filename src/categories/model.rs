use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub user_id: Option<i64>,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
