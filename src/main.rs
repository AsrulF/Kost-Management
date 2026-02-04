use axum::{Router, Extension};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

mod config;
mod models;
mod utils;
mod middlewares;
mod schemas;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    // Load the environment file
    dotenv().ok();

    // Try to connect to database
    let db = config::database::connect().await;

    // Cors configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    //Make basic route
    let app = Router::new()
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_route::user_routes())
        .merge(routes::kost_route::kost_route())
        .merge(routes::room_route::room_route())
        .layer(Extension(db))
        .layer(cors);

    //Take port from environment variable,
    let port = std::env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    //Server address
    let addrs = SocketAddr::from(([127,0,0,1], port));

    //Print server to console
    println!("Server is running on http:://{}", addrs);

    //Run the server
    axum::serve(
        tokio::net::TcpListener::bind(addrs).await.unwrap(),
        app
        )
        .await
        .unwrap();
}
