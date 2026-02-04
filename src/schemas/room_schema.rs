use serde::{
    Serialize,
    Deserialize
};

use chrono::{
    DateTime,
    Utc,
};

use uuid::Uuid;
use validator::Validate;
use sqlx::Type;

#[derive(Deserialize, Validate)]
pub struct RoomNewRequest {
    #[validate(range(min = 1, message = "Room number cannot be empty"))]
    pub room_number: u32,
    pub room_vacancy: RoomStatus,
}

#[derive(Debug, Serialize)]
pub struct RoomNewResponse {
    pub id: Uuid,
    pub kost_id: Uuid,
    pub room_number: u32,
    pub room_vacancy: RoomStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Validate)]
pub struct RoomUpdateRequest {
    #[validate(range(min = 1, message = "Room number cannot be empty"))]
    pub room_number: u32,
    pub room_vacancy: RoomStatus,
}

#[derive(Debug, Serialize)]
pub struct RoomUpdateResponse {
    pub id: Uuid,
    pub kost_id: Uuid,
    pub room_number: u32,
    pub room_vacancy: RoomStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Type, Serialize, Deserialize)]
#[sqlx(type_name = "ENUM")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoomStatus {
    Available,
    Occupied,
    Maintenance,
}



