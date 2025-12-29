use crate::utils::{
    mod_auth::{LoginRequest, login}, 
    mod_data::{Kost, Kosts}, 
    mod_user::{User, Users, Role}, 
    mod_response::*,
    state::AppState, 
};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use actix_cors::Cors;
use std::sync::{Mutex, Arc};

mod utils {
    pub mod mod_data;
    pub mod mod_user;
    pub mod mod_auth;
    pub mod mod_response;
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

#[get("/app-data")]
async fn get_app_data(state: web::Data<AppState>) -> impl Responder {
    let users = state.user_db.lock().unwrap();
    let kosts = state.kosts_db.lock().unwrap();

    let users_dtos = users
        .list
        .iter()
        .map(|u| UserDto {
            username: u.username.clone(),
            user_id: u.user_id,
            user_role: {
                match u.user_role {
                    Role::Admin => "Admin".to_string(),
                    Role::NotAdmin => "Member".to_string()
                }
            }
        })
        .collect::<Vec<_>>();

    let kosts_dtos = kosts
        .kost_database
        .iter()
        .map(|k| KostDto {
            kost_id: k.user_id,
            kost_rooms: k.rooms.len() as u8,
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(AppStateRespose {
        users: users_dtos,
        kosts: kosts_dtos
    })
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        user_db: Arc::new(Mutex::new(Users::new())),
        kosts_db: Arc::new(Mutex::new(Kosts::new())),
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