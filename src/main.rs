use crate::utils::mod_data::KostRooms;

mod utils {
    pub mod mod_data;
}

fn main() {
    let mut rooms = KostRooms::new();
    KostRooms::input_tenant(&mut rooms, "daus".to_string(), "0821".to_string(), "adw@mail.com".to_string());
    KostRooms::room_status(&rooms);
}
