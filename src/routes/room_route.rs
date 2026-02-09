use axum::{
    Router,
    middleware::{self, from_fn},
    routing::{delete, get, post, put},
};

// Import room handler
use crate::handlers::room_handler::{
    create_room,
    get_all_rooms, 
    get_room_by_id, 
    update_room,
    delete_room,
};

// Import auth middleware
use crate::middlewares::auth_middleware::auth;

// Import permission middleware
use crate::middlewares::permission_middleware::require_permission_owner;

// Handler to create new room
pub fn room_route() -> Router {
    Router::new()
        // POST /api/kosts/{id} -> Create a new room for the kost
        .route("/api/kosts/{kost_id}/rooms",
        post(create_room)
            .layer(from_fn(require_permission_owner))
        )
        // GET /api/kosts/{id} -> Get all rooms
        .route("/api/kosts/{kost_id}/rooms", 
        get(get_all_rooms)
            .layer(from_fn(require_permission_owner))
        )
        // GET /api/kosts/{kost_id}/rooms/{room_id} => Get room by id
        .route(
            "/api/kosts/{kost_id}/rooms/{room_id}",
            get(get_room_by_id)
                .layer(from_fn(require_permission_owner)) 
        )
        // PUT /api/kosts/{kost_id}/rooms/{room_id} => update room data
        .route(
            "/api/kosts/{kost_id}/rooms/{room_id}", 
            put(update_room))
                .layer(from_fn(require_permission_owner)
        )
        // DELETE /api/kosts/{kost_id}/rooms/{room_id} => delete room
        .route(
            "/api/kosts/{kost_id}/rooms/{room_id}", 
            delete(delete_room))
                .layer(from_fn(require_permission_owner)
        )
        .layer(from_fn(auth))
}