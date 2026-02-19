use tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt};
use tudu::db::create_pool;
use tudu::{
    build_app, build_state,
    config::{host_url, port},
};

#[tokio::main]
async fn main() -> () {
    dotenvy::dotenv().ok();

    registry().with(tracing_subscriber::fmt::layer()).init();

    // get database pool con
    let pool = match create_pool().await {
        Ok(v) => v,
        Err(e) => panic!("Error while creating a sqlx pool: {}", e),
    };
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let state = build_state(pool).await;
    let app = build_app(state);

    let url = host_url() + ":" + port().as_str();
    let listener = tokio::net::TcpListener::bind(url.clone()).await.unwrap();

    println!("Server running on http://{}", url);
    axum::serve(listener, app).await.unwrap();
}
