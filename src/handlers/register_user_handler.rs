use axum::{
    Extension,
    Json,
    http::StatusCode,
};
use sqlx::MySqlPool;
use bcrypt::hash;
use validator::Validate;
use std::collections::HashMap;
use serde_json::{json, Value};
use uuid::Uuid;

//Import register user request and response schema
use crate::schemas::register_schema::{
    RegisterRequest,
    RegisterResponse
};

//Import API response from utils
use crate::utils::response::ApiResponse;

pub async fn register(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<RegisterRequest>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    //Validate request
    if let Err(e) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        //Collect all the errors from validate
        for (field, errors) in e.field_errors() {
            let messages = errors
                .iter()
                .filter_map(|e|e.message.as_ref())
                .map(|m| m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), messages);
        }

        return (
            //Send 422 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse{
                status: false,
                message: "Failed to validate".to_string(),
                data: Some(json!(field_errors))
            }),
        );
    }

    //Hash password with Bcrypt
    let password = match hash(payload.password, 10) {
        Ok(hashed) => hashed,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to encrypt the password",
                )),
            );
        }
    };


    //Insert user's data to database
    let user_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO users (id, name, email, password) VALUES (?, ?, ?, ?)",
        user_id,
        payload.name,
        payload.email,
        password
    )
    .execute(&db)
    .await;

    match result {
        Ok(result) => {
            //Get user data by user id
            let user = sqlx::query!(
                r#"
                SELECT id AS "id: Uuid", name, email, created_at, updated_at
                FROM users
                WHERE id = ?
                "#,
                user_id
            )
            .fetch_one(&db)
            .await;
            
            match user {
                Ok(user) => {
                    let response = RegisterResponse {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    };

                    (
                        //Send 201 response Created
                        StatusCode::CREATED,
                        Json(ApiResponse::succes(
                            "Register success", 
                            json!(response)    
                        ))
                    )
                }

                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Failed to fetch user's data"
                    ))
                )
            }
        }

        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                (
                    StatusCode::CONFLICT,
                    Json(ApiResponse::error(
                        "Email has been registered"
                    )),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        e.to_string().as_str(),
                    ))
                )
            }
        }
    }
}