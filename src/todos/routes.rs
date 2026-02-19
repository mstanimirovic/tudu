use crate::{auth::middleware::auth_middleware, state::AppState};

use super::handlers;
use axum::{Router, middleware, routing::*};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_todo).get(handlers::list_todos))
        .route(
            "/{id}",
            get(handlers::get_todo)
                .put(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
        .layer(middleware::from_fn(auth_middleware))
}
