// src/error.rs (FINAL)

use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

// --- AppError Definition ---

#[derive(Debug)]
pub enum AppError {
    Sqlx(sqlx::Error),
    NotFound,
    Message(String),
    Unauthorized, // For missing/invalid JWT token
    Reqwest(reqwest::Error),
}

// Implement From traits (for easy conversion)
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self { AppError::Sqlx(e) }
}
impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self { AppError::Reqwest(e) }
}


// --- AppError to HTTP Response Conversion ---

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Sqlx(e) => {
                eprintln!("SQLx Error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred.".to_string())
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Resource not found".to_string())
            }
            AppError::Message(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::Reqwest(e) => {
                (StatusCode::BAD_GATEWAY, format!("External service failed: {}", e))
            }
            // CRITICAL: Explicitly send 401 when authorization fails
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Missing or invalid authorization token.".to_string())
            }
        };

        // Standard JSON error body structure
        let body = Json(json!({
            "status_code": status.as_u16(),
            "message": error_message,
        }));

        (status, body).into_response()
    }
}