use super::jwt::verify_jwt;
use crate::{error::AppError, state::AppState};
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
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

    if state.users_repo.find_by_id(claims.sub).await?.is_none() {
        return Err(AppError::Unauthorized);
    }

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}
