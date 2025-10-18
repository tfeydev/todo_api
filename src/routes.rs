// src/routes.rs
use axum::{
    extract::{State, Path},
    response::IntoResponse,          // For response types
    http::StatusCode,                // Only ONE StatusCode import!
    routing::{get, put},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use reqwest;                         // For HTTP calls to Julia
use serde_json::json;                // For simple JSON creation

// Import error handling
use crate::error::AppError; 

// Import authentication middleware (the Extractor)
use crate::middleware::AuthUser;

// --- DTOs and Models ---

/// Type alias for cleaner return values
pub type AppResult<T> = Result<T, AppError>;

/// Response structure coming from the Julia service
#[derive(serde::Deserialize)]
struct ScoreResponse {
    score: f64,
}

/// Todo model for SQLx and JSON (CORRECTED SYNTAX)
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,  // <-- CORRECTED: Must be i32
    pub title: String,
    pub done: bool,
    pub score: f64, // Julia's Output
}

/// Input structure for creating a Todo
#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}

/// Input structure for updating a Todo
#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub done: bool,
}

// --- Route Handlers ---

/// --- Protected Route ---
/// Example route accessible only with a valid JWT.
pub async fn protected_route(AuthUser(email): AuthUser) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Welcome, {email}! You have access to this protected route."),
    )
}


/// --- GET /todos --- (Actual path: /todos)
/// Retrieves all Todos from the database
pub async fn get_todos(
    AuthUser(_): AuthUser, // Secured, must be before State(pool)
    State(pool): State<PgPool>,
) -> AppResult<Json<Vec<Todo>>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, done, score FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?; // Sqlx Error -> AppError::Sqlx

    Ok(Json(todos))
}

/// --- POST /todos --- (Actual path: /todos)
/// Creates a new Todo, calls Julia for the "score"
pub async fn create_todo(
    AuthUser(_): AuthUser, // Secured, must be before State(pool)
    State(pool): State<PgPool>,
    Json(payload): Json<NewTodo>,
) -> AppResult<StatusCode> {
    
    // --- JULIA INTEGRATION ---
    let client = reqwest::Client::new();
    let julia_response = client
        .post("http://127.0.0.1:8081/score") 
        .json(&json!({ "title": payload.title }))
        .send()
        .await
        .map_err(|e| AppError::Message(format!("Julia service request failed: {}", e)))?;

    let julia_response = julia_response.error_for_status()?;
    let score_data = julia_response.json::<ScoreResponse>().await?;
    let score = score_data.score;

    // --- SAVE TO DB ---
    sqlx::query("INSERT INTO todos (title, done, score) VALUES ($1, FALSE, $2)")
        .bind(&payload.title)
        .bind(score)
        .execute(&pool)
        .await?;

    Ok(StatusCode::CREATED)
}

/// --- PUT /todos/:id --- (Actual path: /todos/:id)
/// Updates an existing Todo and re-calls Julia
pub async fn update_todo(
    AuthUser(_): AuthUser, // Secured, must be before State(pool)
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> AppResult<StatusCode> {
    
    // --- Re-call Julia ---
    let client = reqwest::Client::new();
    let julia_response = client
        .post("http://127.0.0.1:8081/score")
        .json(&json!({ "title": payload.title }))
        .send()
        .await
        .map_err(|e| AppError::Message(format!("Julia service request failed: {}", e)))?;

    let julia_response = julia_response.error_for_status()?;
    let score_data = julia_response.json::<ScoreResponse>().await?;
    let new_score = score_data.score;

    // --- DB Update ---
    let result = sqlx::query("UPDATE todos SET title = $1, done = $2, score = $3 WHERE id = $4")
        .bind(&payload.title)
        .bind(&payload.done)
        .bind(new_score)
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// --- DELETE /todos/:id --- (Actual path: /todos/:id)
/// Deletes a Todo
pub async fn delete_todo(
    AuthUser(_): AuthUser, // Secured, must be before State(pool)
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// --- Create Todo Router ---
/// Note: Since this router is nested under "/todos" in main.rs,
/// the routes here are defined relative to the root of the nested path.
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        // Path / corresponds to /todos
        .route("/", get(get_todos).post(create_todo)) 
        // Path /:id corresponds to /todos/:id
        .route("/:id", put(update_todo).delete(delete_todo)) 
        .with_state(pool)
}