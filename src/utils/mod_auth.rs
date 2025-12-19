use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey,Header};
use chrono::{Utc, Duration};
use dotenvy::dotenv;
use std::env;

use crate::utils::mod_user::{LoginError, User, Users};

pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debu, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u32,
    pub username: String,
    pub exp: usize,
}

// Helper function for Login
pub fn login(
    database: &Users,
    username: String,
    password: String,
) -> Result<AuthResponse, LoginError> {
    todo!()
}

pub fn create_jwt(user: &User) -> Result<String, LoginError> {
    dotenv().ok();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.user_id,
        username: user.username.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(&jwt_secret),
    )
    .map_err(|_| LoginError::TokenCreationError)
} 