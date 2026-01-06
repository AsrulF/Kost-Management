use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn connect() -> MySqlPool {

    // Take DATABASE_URL from environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");

    // Try to connect to database
    match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Database connected succesfully");
            pool
        },
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    }
}