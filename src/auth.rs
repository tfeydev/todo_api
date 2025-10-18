// src/auth.rs (Login Handler: CORRECTED for Dynamic JWT Generation)
use axum::{
    routing::post,
    extract::State,
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey}; 
use std::env;

// --- DTOs ---

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

// Claims structure (Must match middleware.rs)
#[derive(Debug, Serialize, Deserialize)] 
pub struct Claims {
    pub sub: String,  // Subject (the user’s email)
    pub exp: usize,   // Expiration timestamp
}


// --- Handler ---

/// Mock login handler that simulates verification (DB check) and token generation.
pub async fn login_handler(
    // Retain State for PgPool to match expected signature
    State(_pool): State<PgPool>, 
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    println!("MOCK LOGIN: Attempt for user: {}", payload.email);

    // MOCK VERIFICATION: In a real app, this is where you query the DB 
    // and verify the password hash.
    if payload.email == "thor@techthor.com" && payload.password == "secret123" {
        
        // 1. Prepare JWT Claims (24-hour expiration)
        let now = chrono::Utc::now();
        let expiration = now + chrono::Duration::hours(24);
        
        let claims = Claims {
            sub: payload.email.clone(),
            exp: expiration.timestamp() as usize,
        };

        // 2. Load the JWT_SECRET (MUST MATCH middleware.rs for validation)
        let secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set in the .env file and loaded!");
        
        // 3. Encode the token
        let token_result = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes())
        );

        let new_token = match token_result {
            Ok(t) => t,
            Err(e) => {
                 eprintln!("Token creation failed: {}", e);
                 return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Failed to generate token"}))
                ).into_response()
            },
        };

        // 4. Send the dynamically created and signed token
        return (
            StatusCode::OK,
            Json(serde_json::json!({"token": new_token}))
        ).into_response();
    }

    // Default: Unauthorized
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"error": "Invalid credentials"}))
    ).into_response()
}

// --- Router ---

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .with_state(pool)
}