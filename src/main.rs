use crate::utils::{mod_data::Kost, mod_user::Users, state::AppState};
use actix_web::{App, HttpServer, get, post, web};
use std::sync::Mutex;

mod utils {
    pub mod mod_data;
    pub mod mod_user;
    pub mod state;
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        kost_db: Mutex::new(Kost::new(5)),
        user_db: Mutex::new(Users::new()),
    });
    
    HttpServer::new(move ||{
        App::new()
            .app_data(state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}