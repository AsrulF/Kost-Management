use axum::{
    Router,
    routing::{get, post},
    middleware
};

//Import user handler
use crate::handlers::user_handler::{
    index,
    store,
};

//Import auth middleware
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        // GET /api/users -> list all users
        .route("/api/users", get(index))
        // POST /api/users -> create new user
        .route("/api/users", post(store))
        // Guard protector for all route above, make sure user must logged in
        .layer(middleware::from_fn(auth))
}