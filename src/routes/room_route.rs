use axum::{
    Router,
    middleware::{self, from_fn},
    routing::{delete, get, post, put},
};

// Import room handler
use crate::handlers::room_handler::{
    create_room,
};

// Import auth middleware
use crate::middlewares::auth_middleware::auth;

// Import permission middleware
use crate::middlewares::permission_middleware::require_permission_owner;

// Handler to create new room
pub fn room_route() -> Router {
    Router::new()
        // POST /api/kosts/{id} -> Create a new room for the kost
        .route("/api/kosts/{id}",
        post(create_room)
            .layer(from_fn(require_permission_owner))
        .layer(from_fn(auth))
    )
}