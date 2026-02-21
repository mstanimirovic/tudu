use std::env;

pub fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string())
}

pub fn host_url() -> String {
    env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}

pub fn port() -> String {
    env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string())
}

pub fn jwt_secret() -> String {
    env::var("JWT_SECRET").expect("Please set jwt secret env var")
}
