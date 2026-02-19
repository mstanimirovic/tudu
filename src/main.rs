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
    categories::repository::CategoryRepository,
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

pub fn build_app(app_state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(health))
        .nest("/auth", auth::routes::routes())
        .nest("/api/users", users::routes::routes())
        .nest("/api/todos", todos::routes::routes())
        .nest("/api/categories", categories::routes::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}

pub async fn build_state(pool: Pool<Sqlite>) -> AppState {
    AppState::new(
        UserRepository::new(pool.clone()),
        TodoRepository::new(pool.clone()),
        CategoryRepository::new(pool.clone()),
    )
}

#[tokio::main]
async fn main() -> () {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // get database pool con
    let pool = match db::create_pool().await {
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
