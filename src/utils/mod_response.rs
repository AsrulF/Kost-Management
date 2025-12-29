use serde::Serialize; 

#[derive(Serialize)]
pub struct UserDto {
    pub username: String,
    pub user_id: u64,
    pub user_role: String,
}

#[derive(Serialize)]
pub struct KostDto {
    pub kost_id: u64,
    pub kost_rooms: u8,
}

#[derive(Serialize)]
pub struct AppStateRespose {
    pub users: Vec<UserDto>,
    pub kosts: Vec<KostDto>,
}