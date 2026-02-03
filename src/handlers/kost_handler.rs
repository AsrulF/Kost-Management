use std::collections::HashMap;

use axum::{
    Extension,
    Json,
    http::StatusCode,
    extract::Path,
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
    KostNewResponse,
    KostUpdateRequest,
    KostUpdateResponse,
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

    // Check kost_name uniqueness
    let kost_name_exist = sqlx::query!(
        "SELECT kost_name FROM Kosts WHERE user_id = ? AND kost_name = ?",
        kost_user_id,
        payload.kost_name,
    )
    .fetch_optional(&db)
    .await;

    if let Ok(Some(_)) = kost_name_exist {
        return (
            // Send 409 response Conflict
            StatusCode::CONFLICT,
            Json(ApiResponse::error(
                "Kost name already exist"
            ))
        );
    }

    let result = sqlx::query!(
        "INSERT INTO Kosts (id, user_id, kost_name, kost_address, kost_contact, kost_desc) VALUES (?, ?, ?, ?, ?, ?)",
        kost_id,
        kost_user_id,
        payload.kost_name,
        payload.kost_address,
        payload.kost_contact,
        payload.kost_desc,
    )
    .execute(&db)
    .await;

    match result {
        Ok(_result) => {
            // Get newly created kost
            let kost = sqlx::query_as!(
                Kost,
                r#"
                SELECT id AS "id: Uuid", user_id AS "user_id: Uuid", kost_name, kost_address, kost_contact, kost_desc, created_at, updated_at
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
                        kost_desc: kost.kost_desc,
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
    Extension(claims): Extension<Claims>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Get all kost data
    if claims.role == "ADMIN" {
        let kosts = match sqlx::query_as!(
            Kost,
            r#"
            SELECT id AS "id: Uuid", user_id AS "user_id: Uuid", kost_name, kost_address, kost_contact, kost_desc, created_at, updated_at
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
    } else {
        let kosts = match sqlx::query_as!(
            Kost,
            r#"
            SELECT id AS "id: Uuid", user_id AS "user_id: Uuid", kost_name, kost_address, kost_contact, kost_desc, created_at, updated_at
            FROM Kosts
            WHERE user_id = ?
            ORDER BY kost_name DESC
            "#,
            claims.sub
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
                json!(kosts)))
        )
        
    }
}

// Handler to get kost detail
pub async fn get_kost_by_id(
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Extension(db): Extension<MySqlPool>
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Get kost data by kost id
    let kost = match sqlx::query!(
        r#"
            SELECT 
                id AS "id: Uuid", 
                user_id AS "user_id: Uuid",
                kost_name,
                kost_address,
                kost_contact,
                kost_desc,
                created_at,
                updated_at
            FROM Kosts
            WHERE id = ?
        "#,
        id
    )
    .fetch_one(&db)
    .await
    {
        Ok(kost) => kost,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "Kost not found"
                ))
            );
        },
        Err(e) => {
            eprintln!("{}", e);
            return (
                // Send 500 response Iternal Server Error,
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Failed to get kost data"
                ))
            );
        }
    };

    // Guard, only matched user id can access the kost
    if kost.user_id != claims.sub {
        return (
            // Send 401 response Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Only owner can access this kost",
            ))
        )
    }

    let response = KostNewResponse {
        id: kost.id,
        user_id: kost.user_id,
        kost_name: kost.kost_name,
        kost_address: kost.kost_address,
        kost_contact: kost.kost_contact,
        kost_desc: kost.kost_desc,
        created_at: kost.created_at,
        updated_at: kost. updated_at,
    };

    (
        // Send 200 response Ok
        StatusCode::OK,
        Json(ApiResponse::success(
            "Kost details", 
            json!(response))),
    )
}

// Handler to update kost data
pub async fn update_kost(
    Extension(db): Extension<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<KostUpdateRequest>,
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
            // Send 422 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse{
                status: false,
                message: "Failed to validate".to_string(),
                data: Some(json!(field_errors))
            })
        );
    }

    // Check if the kost exist
    let kost = match sqlx::query!(
        r#"
        SELECT id, user_id AS "user_id: Uuid"
        FROM Kosts
        WHERE id = ?
        "#,
        id
    ) 
    .fetch_one(&db)
    .await
    {
        Ok(kost) => kost,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 404 response not found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "Kost with provided id is not found"
                ))
            );
        },
        Err(_) => {
            return (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Server Error"
                ))
            );
        }
    };

    // Guard, only owner can update the kost
    if kost.user_id != claims.sub {
        return (
            // Send 401 response Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Only owner can update the kost",
            ))
        );
    }

    // Update kost data
    let result = sqlx::query!(
        "
        UPDATE Kosts
        SET kost_name = ?, kost_address = ?, kost_contact = ?, kost_desc = ?
        WHERE id = ?
        ",
        payload.kost_name,
        payload.kost_address,
        payload.kost_contact,
        payload.kost_desc,
        id,
    )
    .execute(&db)
    .await;

    if let Err(e) = result {
        return (
            // Send 500 response Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                "Server error",
            ))
        );
    }

    // Get kost new data
    let updated_kost = sqlx::query!(
        r#"
        SELECT 
            id AS "id: Uuid",
            user_id AS "user_id: Uuid",
            kost_name,
            kost_address,
            kost_contact,
            kost_desc,
            created_at,
            updated_at
        FROM Kosts
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&db)
    .await;

    match updated_kost {
        Ok(updated_kost) => {
            let response = KostUpdateResponse {
                id: updated_kost.id,
                user_id: updated_kost.user_id,
                kost_name: updated_kost.kost_name,
                kost_address: updated_kost.kost_address,
                kost_contact: updated_kost.kost_contact,
                kost_desc: updated_kost.kost_desc,
                created_at: updated_kost.created_at,
                update_at: updated_kost.updated_at,
            };

            return (
                // Send 200 response Ok
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Kost updated successfully", 
                    json!(response))),
            );
        },
        Err(_) => {
            return (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Server Error",
                ))
            );
        }
    }
}

// Handler to delete kost
pub async fn delete_kost(
    Path(id): Path<Uuid>,
    Extension(db): Extension<MySqlPool>,
    Extension(claims): Extension<Claims>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Check if the kost exist
    let kost = match sqlx::query!(
        r#"SELECT id, user_id AS "user_id: Uuid" FROM Kosts WHERE id = ?"#,
        id
    )
    .fetch_one(&db)
    .await 
    {
        Ok(kost) => kost,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "Kost with provided id is not found",
                ))
            );
        },
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

    // Guard, so only kost owner can delete the kost
    if kost.user_id != claims.sub {
        return (
            // Send 401 response Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Only kost owner can delete this kost"
            ))
        );
    }

    // Delete the kost
    let result = sqlx::query!(
        "DELETE FROM Kosts WHERE id = ?",
        id
    )
    .execute(&db) 
    .await;

    match result {
        Ok(_) => {
            return (
                // Send 200 response Ok
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Kost deleted successfully", 
                    json!(null)))
            );
        },
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
}