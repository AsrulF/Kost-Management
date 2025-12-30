use serde::{Deserialize, Serialize};
use uuid::Uuid; 

// Handle AppState for frontend

#[derive(Serialize)]
pub struct UserDto {
    pub username: String,
    pub user_id: Uuid,
    pub user_role: String,
}

#[derive(Serialize)]
pub struct KostDto {
    pub kost_id: Uuid,
    pub kost_rooms: u8,
}

#[derive(Serialize)]
pub struct AppStateRespose {
    pub users: Vec<UserDto>,
    pub kosts: Vec<KostDto>,
}

// Handle create new user from frontend

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
    pub user_role: String,
}