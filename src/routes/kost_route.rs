use axum::{
    Router,
    middleware::{self, from_fn},
    routing::{get, post, put},
};

// Import kost handler
use crate::handlers::kost_handler::{
    create_new_kost,
    get_all_kosts,
    get_kost_by_id,
    update_kost,
};

// Import auth middleware
use crate::middlewares::auth_middleware::auth;

// Import permission middleware
use crate::middlewares::permission_middleware::require_permission_owner;

pub fn kost_route() -> Router {
    Router::new()
        // POST /api/kosts/{id} -> create new kost
        .route(
            "/api/kosts", 
            post(create_new_kost)
                .layer(from_fn(require_permission_owner))
        )
        .route(
            "/api/kosts", 
            get(get_all_kosts)
                .layer(from_fn(require_permission_owner))
        )
        .route(
            "/api/kosts/{id}", 
            get(get_kost_by_id)
                .layer(from_fn(require_permission_owner))
        )
        .route(
            "/api/kosts/{id}", 
            put(update_kost)
                .layer(from_fn(require_permission_owner))
        )
        .layer(middleware::from_fn(auth))
}