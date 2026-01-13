use axum::{
    Router,
    middleware,
    routing::{post},
};

// Import kost handler
use crate::handlers::kost_handler::{
    create_new_kost,
};

// Import auth middleware
use crate::middlewares::auth_middleware::auth;

pub fn kost_route() -> Router {
    Router::new()
        // POST /api/kosts/{id} -> create new kost
        .route("/api/kosts/{id}", post(create_new_kost))
        .layer(middleware::from_fn(auth))
}