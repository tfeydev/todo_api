use thiserror::Error;
use axum::response::{Response, IntoResponse};
use axum::http::StatusCode;
use serde_json::json;

/// Central error type for the application.
/// It wraps errors from SQLx (database) and Reqwest (Julia service).
#[derive(Debug, Error)]
pub enum AppError {
    
    // 1. Database Errors (e.g., connection, query failure, decode issues)
    // The #[from] attribute automatically converts sqlx::Error into AppError::Sqlx.
    #[error("Database error occurred: {0}")]
    Sqlx(#[from] sqlx::Error),
    
    // 2. Reqwest Errors (Network errors when contacting Julia, including connection failures,
    // timeout, or errors during JSON parsing/status code checks).
    #[error("Network or parsing error with Julia service: {0}")]
    Reqwest(#[from] reqwest::Error),
    
    // 3. Custom Error: If a requested resource is not found (e.g., Todo item by ID)
    #[error("Resource not found.")]
    NotFound,
}

// Implement conversion from AppError to an HTTP Response for Axum.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        // Define the HTTP status code and a user-friendly message for the client.
        let (status, client_message) = match self {
            
            // 1. SQLx Errors
            AppError::Sqlx(e) => {
                // Log the detailed error internally for debugging.
                eprintln!("SQLx Error: {:?}", e); 
                (StatusCode::INTERNAL_SERVER_ERROR, "A server error occurred during database operation.".to_string())
            }
            
            // 2. Reqwest Errors (Julia communication)
            AppError::Reqwest(e) => {
                // Log the detailed error internally.
                eprintln!("Reqwest Error: {:?}", e); 
                // Use SERVICE_UNAVAILABLE if the dependency (Julia) is down or unresponsive.
                (StatusCode::SERVICE_UNAVAILABLE, "The external scoring service is currently unreachable or returned an invalid response.".to_string())
            }

            // 3. Not Found Errors (e.g., requested ID doesn't exist)
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "The requested resource was not found.".to_string())
            }
        };

        // Return a structured JSON response to the client.
        (
            status,
            axum::Json(json!({
                "error": client_message,
            })),
        ).into_response()
    }
}
