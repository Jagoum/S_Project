use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Url {
    pub id : i32,
    pub long_url: String,
    pub short_url: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUrl {
    pub long_url: String,
}