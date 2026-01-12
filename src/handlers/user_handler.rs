use std::collections::HashMap;

use axum::{
    Extension,
    Json,
    http::StatusCode,
    extract::Path,
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
    UserNewResponse, 
    UserUpdateRequest,
};

//Import API response from utils
use crate::utils::response::ApiResponse;

//Handler to get all users data
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
        Json(ApiResponse::success(
            "User List", 
            json!(users)
        ))
    )
}

//Handler to create new user
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
                        Json(ApiResponse::success(
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

//Handler to get user data by ID
pub async fn get_user_by_id(
    Path(id): Path<Uuid>,
    Extension(db) : Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    //Get user data by id
    let user = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", name, email, created_at, updated_at
        FROM Users
        Where id = ?
        "#,
        id
    ) 
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                //Send 404 responds Not Found
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "User not found"
                ))
            );
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                //Send 500 responds Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to fetch user"
                ))
            );
        }
    };

    let response = UserNewResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success(
            "User Details", 
            json!(response)))
    )
}

#[axum::debug_handler]
pub async fn update_user(
    Extension(db) : Extension<MySqlPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UserUpdateRequest>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    //Validate the request
    if let Err(e) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        for (field, errors) in e.field_errors() {
            let messages = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), messages);
        }

        return (
            //Send 422 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse {
                status: false,
                message: "Failed to validate".to_string(),
                data: Some(json!(field_errors))
            })
        );
    };

    if let Some(password) = &payload.password {
        if !password.is_empty() && password.len() < 6 {
            let mut errors: HashMap<String, Vec<String>> = HashMap::new();
            errors.insert(
                "password".to_string(), 
                vec!["Password must be 6 characters".to_string()]
            );

            return (
                //Send 422 response Unprocessable Entity
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse {
                    status: false,
                    message: "Failed to validate password".to_string(),
                    data: Some(json!(errors)),
                })
            );
        }
    };

    //Check if user exist
    let user_exist = match sqlx::query!(
        "SELECT id FROM Users Where id = ?",
        id
    )
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                //Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "User with provided id is not found"
                ))
            );
        },
        Err(_) => {
            return (
                //Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "System error"
                ))
            );
        }
    };

    //Check email uniqueness
    let email_exists = sqlx::query!(
        "SELECT id FROM Users WHERE email = ? AND id != ?",
        payload.email,
        user_exist.id
    )
    .fetch_optional(&db)
    .await;

    if let Ok(Some(_)) = email_exists {
        return (
            //Send conflict response
            StatusCode::CONFLICT,
            Json(ApiResponse::error(
                "Email has been registered",
            ))
        );
    }

    //Update user
    let result = match &payload.password {
        Some(password) if !password.is_empty() => {
            //Hash password using Bcrypt
            let hashed = match hash(password, 10) {
                Ok(h) => h,
                Err(_) => {
                    return (
                        //Send 500 response Internal Server Error
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            "Failed to encrypt password",
                        ))
                    );
                }
            };
            
            sqlx::query!(
                "UPDATE Users SET name = ?, email = ?, password = ? WHERE id = ?",
                payload.name,
                payload.email,
                hashed,
                id
            )
            .execute(&db)
            .await
        },
        _ => {
            //Update user without password
            sqlx::query!(
                "UPDATE Users SET name = ?, email = ? WHERE id = ?",
                payload.name,
                payload.email,
                id
            )
            .execute(&db)
            .await
        }
    };

    if let Err(_) = result {
        return (
            //Send 500 response Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                "Failed to update user",
            ))
        );
    }

    //Get new user data
    let user = sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", name, email, created_at, updated_at
        FROM Users
        WHERE id =?
        "#,
        id
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
                updated_at: user.updated_at,
            };

            return (
                //Send 200 response Ok
                StatusCode::OK,
                Json(ApiResponse::success(
                    "User updated successfully", 
                    json!(response))),
            );
        },
        Err(_) => {
            return (
                //Send 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to get recent updated user data",
                ))
            );
        }
    }
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
    Extension(db) : Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    //Check user
    let user = match sqlx::query!(
        "SELECT id FROM Users WHERE id = ?",
        id
    )
    .fetch_one(&db)
    .await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                //Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "User with provided id is not found",
                ))
            );
        },
        Err(e) => {
            eprintln!("Database error : {}", e);
            return (
                //Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "System Error"
                ))
            );
        }
    };

    //Delete user from database
    let result = sqlx::query!(
        "DELETE FROM Users WHERE id = ?",
        id
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => (
            //Send 200 response ok
            StatusCode::OK,
            Json(ApiResponse::success(
                "User has been deleted", 
                json!(null)))
        ),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                //Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to delete user",
                ))
            )
        } 
    }
}