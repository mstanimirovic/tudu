use crate::{auth::middleware::auth_middleware, state::AppState};

use super::handlers;
use axum::{Router, middleware, routing::*};
use sqlx::SqlitePool;

pub fn routes() -> Router<AppState> {
    let protected = Router::new()
        .route(
            "/{id}",
            get(handlers::get_user)
                .put(handlers::update_user)
                .delete(handlers::delete_user),
        )
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .route("/", get(handlers::get_all_users))
        .merge(protected)
}
