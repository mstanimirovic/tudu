use sqlx::{postgres::PgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = crate::config::database_url();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
