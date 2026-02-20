pub mod config;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod state;
pub mod util;

use axum::Router;
use sqlx::{Pool, Sqlite};
use tower_http::trace::TraceLayer;

use crate::repositories::{
    category_repo::CategoryRepository, todo_repo::TodoRepository, user_repo::UserRepository,
};
use state::AppState;

pub async fn build_state(pool: Pool<Sqlite>) -> AppState {
    AppState::new(
        UserRepository::new(pool.clone()),
        TodoRepository::new(pool.clone()),
        CategoryRepository::new(pool.clone()),
    )
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .merge(routes::health::routes())
        .nest("/auth", routes::auth::routes())
        .nest("/api/users", routes::user::routes())
        .nest("/api/todos", routes::todo::routes())
        .nest("/api/categories", routes::category::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
