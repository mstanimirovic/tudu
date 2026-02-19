pub mod auth;
pub mod categories;
pub mod config;
pub mod db;
pub mod error;
pub mod health;
pub mod state;
pub mod todos;
pub mod users;

use axum::{Router, routing::get};
use sqlx::{Pool, Sqlite};
use tower_http::trace::TraceLayer;

use categories::repository::CategoryRepository;
use state::AppState;
use todos::repository::TodoRepository;
use users::repository::UserRepository;

pub async fn build_state(pool: Pool<Sqlite>) -> AppState {
    AppState::new(
        UserRepository::new(pool.clone()),
        TodoRepository::new(pool.clone()),
        CategoryRepository::new(pool.clone()),
    )
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::routes::handler))
        .nest("/auth", auth::routes::routes())
        .nest("/api/users", users::routes::routes())
        .nest("/api/todos", todos::routes::routes())
        .nest("/api/categories", categories::routes::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
