use jsonwebtoken::{
    encode, decode, Header, EncodingKey, DecodingKey,
    Validation, errors::Error as JwtError
};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Serialize,Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

//Helper function to generate jwt token
pub fn generate_token(user_id: Uuid) -> Result<String, JwtError> {
    //Set expiration token to 24 hours
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    //Make claim token
    encode(
        &Header::default(), 
        &Claims {
            sub: user_id,
            exp,
        },
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET_KEY")
                .unwrap_or_else(|_| "kost_management".to_string())
                .as_ref()
        )
    )
}

//Helper function to verify JWT token
pub fn verify_token(token: &str) -> Result<Claims, JwtError> {
    //Decode the token and verify
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET_KEY")
                .unwrap_or_else(|_| "kost_management".to_string())
                .as_ref()   
        ),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}