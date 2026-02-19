#![allow(unused)]

use axum::{
    Router,
    extract::{Json, Request},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{Pool, Sqlite};
use std::time::Instant;

use crate::{
    config::{host_url, port},
    error::AppError,
    state::AppState,
    todos::repository::TodoRepository,
    users::repository::UserRepository,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod categories;
mod config;
mod db;
mod error;
mod state;
mod todos;
mod users;

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

async fn health() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
    }))
}

async fn create_pool() -> Pool<Sqlite> {
    let pool = db::create_pool().await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    return pool;
}

async fn create_app() -> Router {
    let pool = create_pool().await;
    let app_state = AppState::new(
        UserRepository::new(pool.clone()),
        TodoRepository::new(pool.clone()),
    );

    let app = Router::new()
        .route("/health", get(health))
        .nest("/auth", auth::routes::routes())
        .nest("/api/users", users::routes::routes())
        .nest("/api/todos", todos::routes::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    return app;
}

#[tokio::main]
async fn main() -> () {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = create_app().await;
    let url = host_url() + ":" + port().as_str();
    let listener = tokio::net::TcpListener::bind(url.clone()).await.unwrap();

    println!("Server running on http://{}", url);
    axum::serve(listener, app).await.unwrap();
}
