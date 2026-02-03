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
use crate::models::room;

// Import room schema
use crate::schemas::room_schema::{
    RoomNewRequest,
    RoomNewResponse,
    RoomUpdateRequest,
    RoomUpdateResponse,
};

// Import Claims from utils
use crate::utils::jwt::Claims;

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

    todo!()

}