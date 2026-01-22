use core::error;

use axum::{
    Json, 
    extract::Request, 
    http::StatusCode, 
    middleware::Next, 
    response::Response
};

// Import Api response
use crate::utils::{jwt::Claims, response::ApiResponse};

// Type alias for permission error
type PermissionsError = (StatusCode, Json<ApiResponse<()>>);

// Middleware permissions
pub async fn require_permission_admin(
    mut req: Request,
    next: Next,
) -> Result<Response, PermissionsError> {
    let admin_permission = vec![
        "kost:create",
        "kost:update",
        "kost:delete",
        "kost:view_all",
        "kost:view",
        "user:create",
        "user:update",
        "user:delete",
        "user:view",
    ];

    let claims = req
        .extensions()
        .get::<Claims>();

    match claims {
        Some(claims) => {
            if !has_permission(claims, &admin_permission) {
                return Err(
                    (
                        StatusCode::FORBIDDEN,
                        Json(ApiResponse::<()>::error(
                            "Only Admin can access"
                        ))
                    )
                );
            }

            Ok(next.run(req).await)
        }
        None => {
            return Err(
                (
                    // Send 401 response Forbidden
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::<()>::error(
                        "Please login first"
                    ))
                )
            )
        }
    }
}

pub fn has_permission(claims: &Claims, permits: &[&str]) -> bool {
    permits
        .iter()
        .all(|p| claims.permissions.iter().any(|cp| cp == p))
}