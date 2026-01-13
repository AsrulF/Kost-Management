use serde::{Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Kost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kost_name: String,
    pub kost_address: String,
    pub kost_contact: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}