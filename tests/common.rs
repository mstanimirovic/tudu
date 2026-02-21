use axum_test::TestServer;
use sqlx::Postgres;
use sqlx::{Pool, postgres::PgPoolOptions};

use tudu::app::build_app;
use tudu::app::build_state;
use tudu::config::database_url;

pub async fn test_pool() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url().as_str())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

pub async fn test_server() -> TestServer {
    let pool = test_pool().await;
    let state = build_state(pool).await;
    let app = build_app(state);
    TestServer::new(app).unwrap()
}
