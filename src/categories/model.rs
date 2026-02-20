#[derive(Debug, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub user_id: Option<i64>,
    pub name: String,
    pub color: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
