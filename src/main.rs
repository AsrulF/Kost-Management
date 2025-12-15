use crate::utils::mod_data::{Kost, KostRooms};
use actix_web::{App, get, post, HttpServer};

mod utils {
    pub mod mod_data;
    pub mod mod_user;
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let kost = Kost::new(5);
    println!("{:#?}", kost);
}
