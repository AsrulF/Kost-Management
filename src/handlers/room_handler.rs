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

// Import room model
use crate::models::room::Room;

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
    Json(payload): Json<RoomNewRequest>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
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