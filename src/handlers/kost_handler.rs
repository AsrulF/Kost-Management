use std::collections::HashMap;

use axum::{
    Extension,
    Json,
    http::StatusCode,
};
use sqlx::MySqlPool;
use serde_json::{
    json,
    Value
};
use uuid::Uuid;
use validator::Validate;

// Import kost models
use crate::models::kost::Kost;

// Import kost schema
use crate::schemas::kost_schema::{
    KostNewRequest,
    KostNewResponse
};

use crate::utils::jwt::Claims;
// Import API response form utils
use crate::utils::response::ApiResponse;

// Handler to create new kost
pub async fn create_new_kost(
    Extension(db): Extension<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<KostNewRequest>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Validate the request
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
            //Send 402 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse {
                status: false,
                message: "Failed to validate request".to_string(),
                data: Some(json!(field_errors))
            })
        );
    }

    // Insert new kost to database
    let kost_id = Uuid::new_v4();
    let kost_user_id = claims.sub;

    let result = sqlx::query!(
        "INSERT INTO Kosts (id, user_id, kost_name, kost_address, kost_contact) VALUES (?, ?, ?, ? ,?)",
        kost_id,
        kost_user_id,
        payload.kost_name,
        payload.kost_address,
        payload.kost_contact,
    )
    .execute(&db)
    .await;

    match result {
        Ok(_result) => {
            // Get newly created kost
            let kost = sqlx::query_as!(
                Kost,
                r#"
                SELECT id AS "id: Uuid", user_id AS "user_id: Uuid", kost_name, kost_address, kost_contact, created_at, updated_at
                FROM Kosts
                WHERE id = ?
                "#,
                kost_id
            )
            .fetch_one(&db)
            .await;
            
            match kost {
                Ok(kost) => {
                    let response = KostNewResponse {
                        id: kost.id,
                        user_id: kost.user_id,
                        kost_name: kost.kost_name,
                        kost_address: kost.kost_address,
                        kost_contact: kost.kost_contact,
                        created_at: kost.created_at,
                        updated_at: kost.updated_at,
                    };

                    (
                        // Send 201 response Created
                        StatusCode::CREATED,
                        Json(ApiResponse::success(
                            "Kost created successfully", 
                            json!(response)))
                    )
                },
                Err(_) => (
                    //Send 500 response Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Failed to get kost data"
                    ))
                )
            }
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                //Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref(),
                ))
            )
        }
    }

}

// Handler to get all kost
pub async fn get_all_kosts(
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Get all kost data
    let kosts = match sqlx::query_as!(
        Kost,
        r#"
        SELECT id AS "id: Uuid", user_id AS "user_id: Uuid", kost_name, kost_address, kost_contact, created_at, updated_at
        FROM Kosts
        ORDER BY kost_name DESC
        "#,
    )
    .fetch_all(&db)
    .await
    {
        Ok(kosts) => kosts,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref(),
                ))
            );
        }
    };

    (
        // Send 200 response Ok
        StatusCode::OK,
        Json(ApiResponse::success(
            "Kosts List", 
            json!(kosts),
        ))
    )
}