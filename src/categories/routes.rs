use axum::{Router, middleware, routing::get};

use crate::{
    auth::{self},
    categories::handlers,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::get_categories).post(handlers::create_category),
        )
        .route(
            "/{id}",
            get(handlers::get_category_by_id)
                .patch(handlers::update_category)
                .delete(handlers::delete_category),
        )
        .layer(middleware::from_fn(auth::middleware::auth_middleware))
}
