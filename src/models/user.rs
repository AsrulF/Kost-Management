use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}