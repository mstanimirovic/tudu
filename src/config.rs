use std::env;

pub fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string())
}
