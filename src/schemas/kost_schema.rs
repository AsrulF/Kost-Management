use serde::{
    Serialize,
    Deserialize
};
use chrono::{
    DateTime, Utc
};
use validator::Validate;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct KostNewRequest {
    #[validate(length(min = 3, message = "Kost name must be more than 3 characters"))]
    pub kost_name: String,
    #[validate(required, length(min = 1, message = "Kost address cannot be empty"))]
    pub kost_address: Option<String>,
    #[validate(email(message = "Email is not valid"))]
    pub kost_contact: String,
    #[validate(length(min = 1, message = "Kost description cannot be empty"))]
    pub kost_desc: String,
}

#[derive(Debug, Serialize)]
pub struct KostNewResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kost_name: String,
    pub kost_address: String,
    pub kost_contact: String,
    pub kost_desc: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Validate, Deserialize)]
pub struct KostUpdateRequest {
    #[validate(length(min = 3, message = "Kost new name must be more than 3 characters"))]
    pub kost_name: String,
    #[validate(required, length(min = 1, message = "Kost new address cannot be empty"))]
    pub kost_address: Option<String>,
    pub kost_contact: String,
    #[validate(length(min = 1, message = "Kost description cannot be empty"))]
    pub kost_desc: String,
}

#[derive(Serialize)]
pub struct KostUpdateResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kost_name: String,
    pub kost_address: String,
    pub kost_contact: String,
    pub kost_desc: String,
    pub created_at: Option<DateTime<Utc>>,
    pub update_at: Option<DateTime<Utc>>,
}