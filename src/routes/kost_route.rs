use axum::{
    Router,
    middleware::{self, from_fn},
    routing::{get, post, put},
};

// Import kost handler
use crate::{handlers::kost_handler::{
    create_new_kost,
    get_all_kosts,
    get_kost_by_id,
    update_kost,
}, middlewares::permission_middleware::require_permission_admin};

// Import auth middleware
use crate::middlewares::auth_middleware::auth;

// Import permission middleware
use crate::middlewares::permission_middleware::require_permission_owner;

pub fn kost_route() -> Router {
    Router::new()
        // POST /api/kosts -> create new kost
        .route(
            "/api/kosts", 
            post(create_new_kost)
                .layer(from_fn(require_permission_owner))
        )
        /*  GET /api/kosts -> 
            get all kost, with guard in the handler, if role == "ADMIN", fetch all kosts data
            if not, then fetch all kost data filter by current user id
        */
        .route(
            "/api/kosts", 
            get(get_all_kosts)
                .layer(from_fn(require_permission_owner))
        )
        // GET /api/kosts/{id} -> get kost data by kost id
        .route(
            "/api/kosts/{id}", 
            get(get_kost_by_id)
                .layer(from_fn(require_permission_owner))
        )
        // PUT /api/kosts/{id} -> update kost data
        .route(
            "/api/kosts/{id}", 
            put(update_kost)
                .layer(from_fn(require_permission_owner))
        )
        .layer(middleware::from_fn(auth))
}