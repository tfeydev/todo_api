// src/middleware.rs (JWT Extractor: Validation - FINAL)
use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::env;

use crate::error::AppError; // Import the central error type

/// Claims structure used inside a valid JWT token (Must match auth.rs).
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Suppress warning about 'exp' field not being directly read
pub struct Claims {
    pub sub: String,  // Subject (the user’s email)
    pub exp: usize,   // Expiration timestamp
}

/// Extractor that verifies the JWT token in the `Authorization` header.
pub struct AuthUser(pub String); // Holds the user email extracted from the token

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Get the Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            // Send 401 if header is missing
            .ok_or_else(|| AppError::Unauthorized)? 
            .to_str()
            .map_err(|_| AppError::Unauthorized)?; 

        // 2. Strip the "Bearer " prefix
        let token = auth_header
            .strip_prefix("Bearer ")
            // Send 401 if format is invalid
            .ok_or_else(|| AppError::Unauthorized)?; 

        // 3. Decode and validate the token (Uses the same secret as auth.rs)
        let secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set in the .env file and loaded!");
        
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        // Send 401 if decoding/validation fails (bad signature, expired)
        .map_err(|_| AppError::Unauthorized)?; 

        // 4. Return the extracted subject (user email)
        Ok(AuthUser(token_data.claims.sub))
    }
}