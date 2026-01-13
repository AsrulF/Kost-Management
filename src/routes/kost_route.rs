use axum::{
    Router,
    middleware,
    routing::{post, get},
};

// Import kost handler
use crate::handlers::kost_handler::{
    create_new_kost,
    get_all_kosts,
};

// Import auth middleware
use crate::middlewares::auth_middleware::auth;

pub fn kost_route() -> Router {
    Router::new()
        // POST /api/kosts/{id} -> create new kost
        .route("/api/kosts", post(create_new_kost))
        .route("/api/kosts", get(get_all_kosts))
        .layer(middleware::from_fn(auth))
}