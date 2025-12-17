use crate::utils::mod_data::Kost;
use crate::utils::mod_user::{Users};
use std::sync::Mutex;

pub struct AppState {
    pub kost_db: Mutex<Kost>,
    pub user_db: Mutex<Users>,
}