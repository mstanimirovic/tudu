use crate::{auth::jwt::verify_jwt, error::AppError};
use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};

pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    let token = auth_header.trim_start_matches("Bearer ");

    let claims = verify_jwt(token).map_err(|_| AppError::Unauthorized)?;

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}
