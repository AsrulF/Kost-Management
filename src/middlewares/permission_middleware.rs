use axum::{
    Json, extract::Request, http::StatusCode, middleware::Next, response::Response
};

// Import Api response
use crate::utils::{jwt::Claims, response::ApiResponse};

// Type alias for permission error
type PermissionsError = (StatusCode, ApiResponse<()>);

// Middleware permissions
pub fn require_permission(required: &str) {
    todo!()
}

pub fn has_permission(claims: &Claims, p: &str) -> bool {
    claims.permissions.iter().any(|perm| perm == p)
}