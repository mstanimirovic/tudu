#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: i64,
    pub updated_at: i64,
}
