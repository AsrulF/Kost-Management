use crate::utils::{mod_auth::{LoginRequest, login}, mod_data::Kost, mod_user::Users, state::AppState};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use actix_cors::Cors;
use std::sync::{Mutex, Arc};

mod utils {
    pub mod mod_data;
    pub mod mod_user;
    pub mod mod_auth;
    pub mod state;
}

#[post("/login")]
async fn login_handle(
    form: web::Json<LoginRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    match login(
        &data.user_db.lock().unwrap(), 
        form.username.clone(),
        form.password.clone(),
    ) 
    {
        Ok(auth) => HttpResponse::Ok().json(auth),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        kost_db: Arc::new(Mutex::new(Kost::new(5, 1231234214))),
        user_db: Arc::new(Mutex::new(Users::new())),
    });
    
    HttpServer::new(move ||{
        App::new()
            .app_data(state.clone())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()   
            )
            .service(login_handle)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}