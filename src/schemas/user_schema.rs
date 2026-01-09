use serde::{
    Serialize,
    Deserialize
};
use chrono::{DateTime, Utc};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct UserNewRequest {
    #[validate(length(min = 3, message = "Name cannot be less than 3 characters"))]
    pub name: String,
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserNewResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
