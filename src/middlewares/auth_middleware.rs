use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{HeaderMap, StatusCode},
    Json,  
};

use crate::utils::jwt::verify_token;
use crate::utils::response::ApiResponse;

//Type alias for error response
type AuthError = (StatusCode, Json<ApiResponse<()>>);

//Middleware Authentication
pub async fn auth(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    //Extract token from header
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::error("Token is not found"))
            )
        })?;

    //Token verification
    let claims = verify_token(token)
        .map_err(|e| {
            println!("JWT verification error: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<()>::error("Token is not valid"))
            )
        })?;

    //Save claim in extension for handler to use
    req.extensions_mut().insert(claims);

    //Continue to the next handle
    Ok(next.run(req).await)
}