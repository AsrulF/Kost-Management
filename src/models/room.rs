use serde::Serialize;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schemas::room_schema::RoomStatus;

#[derive(Serialize)]
pub struct Room {
    pub id: Uuid,
    pub kost_id: Uuid,
    pub room_number: u32,
    pub room_vacancy: RoomStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}