use axum::{Router, routing::post};

//Import register handle
use crate::handlers::register_user_handler::register;

//Function to manage route
pub fn auth_routes() -> Router {
    Router::new()
        .route("/api/register", post(register))
}