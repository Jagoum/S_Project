use std::env;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        
        Config {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}