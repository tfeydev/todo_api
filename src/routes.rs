use axum::{
    extract::{State, Path},
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

use reqwest; // HTTP client for Julia communication
use serde_json::json; // To easily construct JSON for Julia

// Import the custom error type
use crate::error::AppError; 

/// Type alias for cleaner return types: Result<T, AppError>
pub type AppResult<T> = Result<T, AppError>;

/// Struct to deserialize the JSON response from the Julia server
#[derive(serde::Deserialize)]
struct ScoreResponse {
    score: f64,
}

/// The Todo model struct, used for sending and receiving todo items.
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    // FIX: Correct type is i32
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub score: f64 // Julia's output field
}

/// Request payload for creating a new todo item.
#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}

/// Request payload for updating an existing todo item.
#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub done: bool,
}

/// Handler to fetch all todo items from the database.
///
/// GET /todos
// Return type uses AppResult to propagate errors safely.
pub async fn get_todos(State(pool): State<PgPool>) -> AppResult<Json<Vec<Todo>>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, done, score FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?; // Sqlx Error -> AppError::Sqlx

    Ok(Json(todos))
}

/// Handler to create a new todo item, contacting Julia for the score.
///
/// POST /todos
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<NewTodo>,
) -> AppResult<StatusCode> {
    
    // --- JULIA INTEGRATION LOGIC ---
    let client = reqwest::Client::new();
    let julia_response = client
        .post("http://127.0.0.1:8081/score") 
        .json(&json!({ "title": payload.title }))
        .send()
        .await?; // Connection/Request Error -> AppError::Reqwest

    let julia_response = julia_response
        .error_for_status()?; // HTTP Status Error (4xx, 5xx) -> AppError::Reqwest

    let score_data = julia_response
        .json::<ScoreResponse>()
        .await?; // JSON Parsing Error -> AppError::Reqwest

    let score = score_data.score;
    // --- END JULIA INTEGRATION LOGIC ---

    // 2. SAVE TO THE DATABASE
    sqlx::query("INSERT INTO todos (title, done, score) VALUES ($1, FALSE, $2)")
        .bind(&payload.title)
        .bind(score) // Bind the score from Julia
        .execute(&pool)
        .await?; // Sqlx Error -> AppError::Sqlx
    
    Ok(StatusCode::CREATED)
}

/// Handler to update an existing todo item by ID.
///
/// PUT /todos/:id
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>, // Path parameter is i32
    Json(payload): Json<UpdateTodo>,
) -> AppResult<StatusCode> {
    
    // --- Call Julia again to update score ---
    let client = reqwest::Client::new();
    let julia_response = client
        .post("http://127.0.0.1:8081/score")
        .json(&json!({ "title": payload.title }))
        .send()
        .await?; // Error -> AppError::Reqwest

    let julia_response = julia_response
        .error_for_status()?; // Error -> AppError::Reqwest

    let score_data = julia_response
        .json::<ScoreResponse>()
        .await?; // Error -> AppError::Reqwest

    let new_score = score_data.score;
    // --- END Julia integration ---

    let result = sqlx::query("UPDATE todos SET title = $1, done = $2, score = $3 WHERE id = $4")
        .bind(&payload.title)
        .bind(&payload.done)
        .bind(new_score)
        .bind(id) // Bind the i32 ID parameter
        .execute(&pool)
        .await?; // Sqlx Error -> AppError::Sqlx

    // If no rows were affected, the ID was invalid -> return 404.
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound); 
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Handler to delete a todo item by ID.
///
/// DELETE /todos/:id
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>, // Path parameter is i32
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?; // Sqlx Error -> AppError::Sqlx

    // If no rows were affected, the ID was invalid -> return 404.
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound); 
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Creates the main application router with all defined endpoints.
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", put(update_todo).delete(delete_todo))
        .with_state(pool)
}
