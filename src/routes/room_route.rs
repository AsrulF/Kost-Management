use axum::{
    Router,
    middleware,
    routing::{get, post, delete, put},
};

// Import room handler
use crate::handlers::room_handler::{
    create_room,
};

// Handler to create new room
pub fn room_route() -> Router {
    Router::new()
        .route("/kost/{id}",
        post(create_room)
    )
}