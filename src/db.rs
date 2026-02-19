use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url = crate::config::database_url();
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
