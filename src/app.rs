use axum::{Router, middleware};
use sqlx::{Pool, Postgres};
use tower_http::trace::TraceLayer;

use crate::categories::repository::CategoryRepository;
use crate::state::AppState;
use crate::todos::repository::TodoRepository;
use crate::users::repository::UserRepository;

pub async fn build_state(pool: Pool<Postgres>) -> AppState {
    AppState::new(
        UserRepository::new(pool.clone()),
        TodoRepository::new(pool.clone()),
        CategoryRepository::new(pool.clone()),
    )
}

pub fn build_app(state: AppState) -> Router {
    let jwt_layer =
        middleware::from_fn_with_state(state.clone(), crate::auth::middleware::auth_middleware);

    Router::new()
        .merge(crate::health::handler::routes())
        .nest("/auth", crate::auth::handler::routes())
        .nest(
            "/users",
            crate::users::handler::routes().layer(jwt_layer.clone()),
        )
        .nest(
            "/todos",
            crate::todos::handler::routes().layer(jwt_layer.clone()),
        )
        .nest(
            "/categories",
            crate::categories::handler::routes().layer(jwt_layer.clone()),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
