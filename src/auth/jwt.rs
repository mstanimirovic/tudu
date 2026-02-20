use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

use crate::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,   // user_id
    pub exp: usize, // expiration time
}

pub fn create_jwt(user_id: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = config::jwt_secret();
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
