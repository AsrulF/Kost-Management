use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
}