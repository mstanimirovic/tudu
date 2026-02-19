#![allow(unused)]

use axum::{
    Router,
    extract::{Json, Request},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use std::time::Instant;

use crate::{
    error::AppError, state::AppState, todos::repository::TodoRepository,
    users::repository::UserRepository,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod config;
mod db;
mod error;
mod state;
mod todos;
mod users;

async fn print_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    println!(
        "{} {} - {} - {:?}",
        method,
        uri.path(),
        response.status().as_u16(),
        duration
    );

    response
}

async fn health() -> Result<Json<String>, AppError> {
    Ok(Json(String::from("ok")))
}

#[tokio::main]
async fn main() -> () {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = db::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app_state = AppState::new(
        UserRepository::new(pool.clone()),
        TodoRepository::new(pool.clone()),
    );

    let app = Router::new()
        .route("/health", get(health))
        .nest("/auth", auth::routes::routes())
        .nest("/api/users", users::routes::routes())
        .nest("/api/todos", todos::routes::routes())
        // .layer(middleware::from_fn(print_request))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
