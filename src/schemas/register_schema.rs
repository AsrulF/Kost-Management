use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate, Debug)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Name must be 3 characters or more"))]
    pub name: String,
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be 6 characters"))]
    pub password: String,
    pub role: Option<RegRole>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub enum RegRole {
    OWNER,
    MEMBER,
}