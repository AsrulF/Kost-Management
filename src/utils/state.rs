use crate::utils::mod_data::Kosts;
use crate::utils::mod_user::{Users};
use std::sync::{Mutex, Arc};

pub struct AppState {
    pub user_db: Arc<Mutex<Users>>,
    pub kosts_db: Arc<Mutex<Kosts>>,
}