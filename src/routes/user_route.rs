use axum::{
    Router, 
    middleware::{self, from_fn}, 
    routing::{delete, get, post, put},
};

//Import user handler
use crate::handlers::user_handler::{
    index,
    store,
    get_user_by_id,
    update_user,
    delete_user,
};

//Import auth middleware
use crate::middlewares::auth_middleware::auth;

// Import permission middleware
use  crate::middlewares::permission_middleware::require_permission_admin;

pub fn user_routes() -> Router {
    Router::new()
        // GET /api/users -> list all users
        .route(
            "/api/users", 
            get(index)
                .layer(from_fn(require_permission_admin))
        )
        // POST /api/users -> create new user
        .route("/api/users", post(store))
        // GET /api/users/{id} -> get user by id
        .route("/api/users/{id}", get(get_user_by_id))
        // PUT /api/users/{id} -> update user's data
        .route("/api/users/{id}", put(update_user))
        // POST /api/users/{id} -> delete user
        .route("/api/users/{id}", delete(delete_user))
        // Guard protector for all route above, make sure user must logged in
        .layer(middleware::from_fn(auth))
}