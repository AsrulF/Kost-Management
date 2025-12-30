use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey,Header};
use chrono::{Utc, Duration};
use dotenvy::dotenv;
use uuid::Uuid;
use std::env;

use crate::utils::mod_user::{LoginError, User, Users};


// Struct for send authorized or not, if authorized, create token via jwt
#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

// Struct for sending json login form
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Struct for create token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub username: String,
    pub exp: usize,
}

// Helper function for Login
pub fn login(
    database: &Users,
    username: String,
    password: String,
) -> Result<AuthResponse, LoginError> {
    let user = database
        .list
        .iter()
        .find(|user| user.username == username && user.password == password)
        .cloned()
        .ok_or_else(|| LoginError::UserNotFound)?;

    let token = create_jwt(&user)?;

    Ok(AuthResponse {
        user,
        token
    })
}

pub fn create_jwt(user: &User) -> Result<String, LoginError> {
    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.user_id.clone(),
        username: user.username.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(&jwt_secret.as_bytes()),
    )
    .map_err(|_| LoginError::TokenCreationError)
} 