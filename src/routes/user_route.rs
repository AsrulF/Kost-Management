use axum::{
    Router,
    routing::{get, post},
    middleware
};

//Import user handler
use crate::handlers::user_handler::{
    index,
    store,
    get_user_by_id,
};

//Import auth middleware
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        // GET /api/users -> list all users
        .route("/api/users", get(index))
        // POST /api/users -> create new user
        .route("/api/users", post(store))
        // GET /api/users/{id} -> get user by id
        .route("/api/users/{id}", get(get_user_by_id))
        // Guard protector for all route above, make sure user must logged in
        .layer(middleware::from_fn(auth))
}