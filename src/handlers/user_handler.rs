use std::collections::HashMap;

use axum::{
    Extension,
    Json,
    http::StatusCode
};

use bcrypt::hash;
use sqlx::MySqlPool;
use serde_json::{
    json, 
    Value
};
use uuid::Uuid;
use validator::Validate;

//Import user models
use crate::models::user::User;

//Import user schema
use crate::schemas::user_schema::{
    UserNewRequest,
    UserNewResponse
};

//Import API response from utils
use crate::utils::response::ApiResponse;

pub async fn index(
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    //Get all user data
    let users = match sqlx::query_as!(
        User,
        r#"
            SELECT id as "id: Uuid", name, email, created_at, updated_at
            FROM Users
            ORDER BY name ASC
        "#
    ) 
    .fetch_all(&db)
    .await
    {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                //Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to get user data"
                )),
            );
        }
    };

    (
        StatusCode::OK,
        Json(ApiResponse::succes(
            "User List", 
            json!(users)
        ))
    )
}

pub async fn store(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<UserNewRequest>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    //Validate request
    if let Err(e) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        //Collect all the errors
        for (field, errors) in e.field_errors() {
            let messages = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), messages);
        }

        return (
            //Send 402 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse{
                status: false,
                message: "Failed to validate".to_string(),
                data: Some(json!(field_errors))
            })
        );
    }

    //Hash password with Bcrypt
    let password = match hash(payload.password, 10) {
        Ok(hashed) => hashed,
        Err(_) => {
            return (
                //Send 500 response Internal Servel Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to encrypt the password"
                ))
            );
        }
    };

    //Insert new user data to database
    let new_user_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO Users (id, name, email, password) VALUES (?, ?, ?, ?)",
        new_user_id,
        payload.name,
        payload.email,
        password
    )
    .execute(&db)
    .await;

    match result {
        Ok(result) => {
            //Get newly created user data
            let user = sqlx::query!(
                r#"
                    SELECT id AS "id: Uuid", name, email, created_at, updated_at
                    FROM Users
                    WHERE id = ?
                "#,
                new_user_id
            )
            .fetch_one(&db)
            .await;

            match user {
                Ok(user) => {
                    let response = UserNewResponse {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        created_at: user.created_at,
                        updated_at: user.updated_at
                    };

                    (
                        //Send 201 response Created
                        StatusCode::CREATED,
                        Json(ApiResponse::succes(
                            "User created succesfully", 
                            json!(response)))
                    )
                },
                Err(_) => (
                    //Send 500 response Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Failed to fetch user data"
                    ))
                )
            }
        },

        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                (
                    //Send 409 response Conflict
                    StatusCode::CONFLICT,
                    Json(ApiResponse::error(
                        "Email has been registered"
                    ))
                )
            } else {
                (
                    //Send 500 response Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        e.to_string().as_ref()
                    ))
                )
            }
        } 
    }
}

