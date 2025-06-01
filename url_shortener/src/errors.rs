use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    #[error("Not found")]
    NotFound,
    #[error("Invalid URL")]
    InvalidUrl,
    // Add other error variants as needed
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidUrl => StatusCode::BAD_REQUEST,
        };
        
        (status, self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;