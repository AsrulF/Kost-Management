use crate::utils::mod_data::Kost;
use crate::utils::mod_user::{Users};
use std::sync::{Mutex, Arc};

pub struct AppState {
    pub kost_db: Arc<Mutex<Kost>>,
    pub user_db: Arc<Mutex<Users>>,
}