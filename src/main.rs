use axum::{Router, Extension};
use dotenvy::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load the environment file
    dotenv().ok();

    //Make basic route
    let app: Router<_> = Router::new()
        .layer(Extension(()));

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
