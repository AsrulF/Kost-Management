use axum::{
    Extension,
    Json,
    http::StatusCode
};
use sqlx::MySqlPool;
use bcrypt::verify;
use validator::Validate;
use std::collections::HashMap;
use serde_json::{Value, json};
use uuid::Uuid;

//Import login schema request and response
use crate::schemas::login_schema::{
    LoginRequest,
    UserResponse,
    LoginResponse
};

//Import utils to generate and validate token
use crate::utils::{
    jwt::generate_token,
    response::ApiResponse,
};

pub async fn login(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<LoginRequest>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    //Validate the request
    if let Err(e) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        //Collect all the errors
        for (field, errors) in e.field_errors() {
            let message = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), message);
        }

        return (
            //Send 422 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse{
                status: false,
                message: "Failed to validate".to_string(),
                data: Some(json!(field_errors)),
            }),
        );
    }

    //Fetch user by email
    let user = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", name, email, password 
        FROM Users 
        WHERE email = ?
        "#,
        payload.email
    )
    .fetch_one(&db)
    .await 
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 401 response Unautorized
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    "Email or password is wrong",
                )),
            );
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                //Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "System Error"
                ))
            );
        }
    };

    //Verify password using bcrypt
    match verify(payload.password, &user.password) {
        Ok(true) => {
            match generate_token(user.id) {
                Ok(token) => {
                    let response = LoginResponse {
                        user: UserResponse {
                            id: user.id,
                            name: user.name,
                            email: user.email,
                        },
                        token,
                    };
                    (
                        StatusCode::OK,
                        Json(ApiResponse::success(
                            "Login success", 
                            json!(response),
                        ))
                    )
                },
                Err(e) => {
                    eprintln!("JWT generation error: {}", e);
                    (
                        //Send 500 response Internal Server Error
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            "Failed to generate token"
                        ))
                    )
                }
            }
        },
        Ok(false) => (

            //Send 401 response Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Email or password is wrong"
            ))
        ),
        Err(_) => (

            //Send 500 response Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                "Failed to verify password"
            ))
        )
    }
}

