use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/actix_rust_db".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .unwrap_or(8081),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        })
    }
}
