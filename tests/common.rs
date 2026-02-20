use axum_test::TestServer;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

use tudu::app::build_app;
use tudu::app::build_state;

pub async fn test_pool() -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
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
