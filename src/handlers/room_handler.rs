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
    Value,
};

use uuid::Uuid;
use validator::Validate;

// Import claims from utils
use crate::utils::jwt::Claims;

// Import room model
use crate::models::room::{
    Room,
    RoomPath,
};

// Import room schema
use crate::schemas::room_schema::{
    RoomNewRequest,
    RoomNewResponse,
    RoomUpdateRequest,
    RoomUpdateResponse,
    RoomStatus,
};

// Import API Response
use crate::utils::response::ApiResponse;

// Handler to create new room
pub async fn create_room(
    Extension(db): Extension<MySqlPool>,
    Path(kost_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RoomNewRequest>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Guard, so only kost owner can access and modify
    let kost = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", user_id AS "user_id: Uuid"
        FROM Kosts
        WHERE id = ?
        "#,
        kost_id
    )
    .fetch_one(&db)
    .await 
    {
        Ok(kost) => kost,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref(),
                ))
            )
        }
    };

    if kost.user_id != claims.sub {
        return (
            // Send 401 Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Only owner can access and modify this kost"
            ))
        );
    }

    // Validate the request
    if let Err(e) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        for (field, errors) in e.field_errors() {
            let messages = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m | m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), messages);
        } 

        return (
            // Send 402 response Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse {
                status: false,
                message: "Failed to validate the request".to_string(),
                data: Some(json!(field_errors))
            })
        );
    }

    // Insert new room to database
    let room_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO Rooms (id, kost_id, room_number, room_vacancy) VALUES (?, ?, ?, ?)",
        room_id,
        kost_id,
        payload.room_number,
        payload.room_vacancy
    )
    .execute(&db)
    .await;

    match result {
        Ok(_result) => {
            // Get newly created room
            let room = sqlx::query_as!(
                Room,
                r#"
                SELECT id AS "id: Uuid", kost_id AS "kost_id: Uuid", room_number AS "room_number: u32", room_vacancy AS "room_vacancy: RoomStatus", created_at, updated_at
                FROM Rooms
                WHERE id = ?
                "#,
                room_id
            )
            .fetch_one(&db)
            .await;

            match room {
                Ok(room) => {
                    let response = RoomNewResponse {
                        id: room.id,
                        kost_id: room.kost_id,
                        room_number: room.room_number,
                        room_vacancy: room.room_vacancy,
                        created_at: room.created_at,
                        updated_at: room.updated_at,
                    };

                    (
                        // Send 200 response Ok
                        StatusCode::OK,
                        Json(ApiResponse::success(
                            "Room created successfully", 
                            json!(response))),
                    )
                },
                Err(_) => (
                    // Send 500 response Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Failed to get new kost data"
                    ))
                )
            }
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref()
                ))
            )
        }
    }

}

// Handler to get all room data
pub async fn get_all_rooms(
    Extension(db): Extension<MySqlPool>,
    Path(kost_id): Path<Uuid>,
    Extension(claims): Extension<Claims>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Get all rooms data
    if claims.role == "ADMIN" {
        let rooms = match sqlx::query_as!(
            Room,
            r#"
            SELECT id AS "id: Uuid", kost_id AS "kost_id: Uuid", room_number AS "room_number: u32", room_vacancy AS "room_vacancy: RoomStatus", created_at, updated_at
            FROM Rooms
            ORDER BY room_number DESC
            "#,
        )
        .fetch_all(&db)
        .await
        {
            Ok(rooms) => rooms,
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
            // Send 200 response OK
            StatusCode::OK,
            Json(ApiResponse::success(
                "Rooms List", 
                json!(rooms)))
        )    
    } else {
        let rooms = match sqlx::query_as!(
            Room,
            r#"
            SELECT id AS "id: Uuid", kost_id AS "kost_id: Uuid", room_number AS "room_number: u32", room_vacancy AS "room_vacancy: RoomStatus", created_at, updated_at
            FROM Rooms
            WHERE kost_id = ?
            ORDER BY room_number ASC
            "#,
            kost_id,
        )
        .fetch_all(&db)
        .await
        {
            Ok(rooms) => rooms,
            Err(e) => {
                eprintln!("Database Error: {}", e);
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
                "Rooms List", 
                json!(rooms)))
        )
    }
}

// Handler to get room by id 
pub async fn get_room_by_id(
    Extension(db): Extension<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Path(path): Path<RoomPath>
) -> (StatusCode, Json<ApiResponse<Value>>) {
    let kost_id = path.kost_id;
    let room_id = path.room_id;

    // Guard
    let kost = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", user_id AS "user_id: Uuid"
        FROM Kosts
        Where id = ?
        "#,
        kost_id,
    )
    .fetch_one(&db)
    .await 
    {
        Ok(kost) => kost,
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

    if kost.user_id != claims.sub {
        return (
            // Send 401 response Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Only owner can access this"
            ))
        );
    }

    // Get room data by id
    let room = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", kost_id AS "kost_id: Uuid", room_number AS "room_number: u32", room_vacancy AS "room_vacancy: RoomStatus", created_at, updated_at
        FROM Rooms
        WHERE id = ? AND kost_id = ?
        "#,
        room_id,
        kost_id,
    ) 
    .fetch_one(&db)
    .await
    {
        Ok(room) => room,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "Room with provided id is not found"
                ))
            );
        },
        Err(e) => {
            eprintln!("Database error: {}",e);
            return (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref(),
                ))
            );
        }
    };

    let response = RoomNewResponse {
        id: room.id,
        kost_id: room.kost_id,
        room_number: room.room_number,
        room_vacancy: room.room_vacancy,
        created_at: room.created_at,
        updated_at: room.updated_at
    };

    (
        // Send 200 response OK
        StatusCode::OK,
        Json(ApiResponse::success(
            "Room Details", 
            json!(response)))
    )
}

// Handler to update room
pub async fn update_room(
    Extension(db): Extension<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Path(path): Path<RoomPath>,
    Json(payload): Json<RoomUpdateRequest>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Guard for kost and room
    let (kost_id, room_id) = (path.kost_id, path.room_id);

    let _kost = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", user_id AS "user_id: Uuid"
        FROM Kosts
        WHERE id = ? AND user_id = ?
        "#,
        kost_id,
        claims.sub
    )
    .fetch_one(&db)
    .await
    {
        Ok(kost) => kost,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Return 404 response Not Found
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

    let _room = match sqlx::query!(
        r#"
        SELECT id as "id: Uuid", kost_id AS "kost_id: Uuid", room_number AS "room_number: u32", room_vacancy AS "room_vacancy: RoomStatus", created_at, updated_at
        FROM Rooms
        WHERE id = ? AND kost_id = ?
        "#,
        room_id,
        kost_id,
    ) 
    .fetch_one(&db)
    .await
    {
        Ok(room) => room,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "Room with provided id is not found"
                ))
            );
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // Return 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref(),
                ))
            );
        }
    };

    // Update kost data
    let result = sqlx::query!(
        "
        UPDATE Rooms
        SET room_number = ?, room_vacancy = ?
        WHERE id = ?
        ",
        payload.room_number,
        payload.room_vacancy,
        room_id
    )
    .execute(&db)
    .await;

    if let Err(e) = result {
        eprintln!("Database error: {}", e);
        return (
            // Send 500 response Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                e.to_string().as_ref(),
            ))
        );
    }

    // Get new room data
    let updated_room = sqlx::query!(
        r#"
        SELECT 
            id AS "id: Uuid",
            kost_id AS "kost_id: Uuid",
            room_number AS "room_number: u32",
            room_vacancy AS "room_vacancy: RoomStatus",
            created_at,
            updated_at
        FROM Rooms
        WHERE id = ?
        "#,
        room_id
    )
    .fetch_one(&db)
    .await;

    match updated_room {
        Ok(updated_room) => {
            let response = RoomUpdateResponse {
                id: updated_room.id,
                kost_id: updated_room.kost_id,
                room_number: updated_room.room_number,
                room_vacancy: updated_room.room_vacancy,
                created_at: updated_room.created_at,
                updated_at: updated_room.updated_at
            };

            return (
                // Send 200 response Ok
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Room updated successfully", 
                    json!(response),
                ))
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

// Handler to delete room
pub async fn delete_room(
    Extension(db): Extension<MySqlPool>,
    Extension(claims): Extension<Claims>,
    Path(path): Path<RoomPath>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Check the kost and room exist
    let (kost_id, room_id) = (path.kost_id, path.room_id);

    let kost = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", user_id AS "user_id: Uuid"
        FROM Kosts
        WHERE id = ? AND user_id = ? 
        "#,
        kost_id,
        claims.sub
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
                    "Kost with provided Id is not found",
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

    let room = match sqlx::query!(
        r#"
        SELECT id AS "id: Uuid", kost_id AS "kost_id: Uuid"
        FROM Rooms
        WHERE id = ? AND kost_id = ?
        "#,
        room_id,
        kost_id,
    ) 
    .fetch_one(&db)
    .await
    {
        Ok(room) => room,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // Send 404 response Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "Room with provided id is not found",
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

    let result = sqlx::query!(
        "
        DELETE FROM Rooms WHERE id = ?
        ",
        room.id
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => {
            return (
                // Send 200 response Ok
                StatusCode::OK,
                Json(ApiResponse::success(
                    "Room deleted successfully", 
                    json!(null)))
            );
        },
        Err(e) => {
            eprintln!("Database Error: {}", e);
            return (
                // Send 500 response Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    e.to_string().as_ref(),
                ))
            );
        }
    }
}