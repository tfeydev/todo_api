use axum::{
    extract::{State, Path},
    http::StatusCode,
   routing::{get, put, post, delete}, // Added post and delete for clarity, though routing is fine
    Json, Router,
};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

use reqwest; // NEW: HTTP client for Julia communication
use serde_json::json; // NEW: To easily construct JSON for Julia

/// Struct to deserialize the JSON response from the Julia server
#[derive(serde::Deserialize)]
struct ScoreResponse {
    score: f64,
}

/// The Todo model struct, used for sending and receiving todo items.
#[derive(Serialize, Deserialize, sqlx::FromRow)] // Added Deserialize for completeness
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub score: f64 // NEW FIELD: Julia's output
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
pub async fn get_todos(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
    // UPDATED SQL: Must select the new 'score' field
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, done, score FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch todos");

    Json(todos)
}

/// Handler to create a new todo item, contacting Julia for the score.
///
/// POST /todos
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<NewTodo>,
) -> StatusCode {
    
    // --- JULIA INTEGRATION LOGIC ---
    // 1. CALL THE JULIA MICROSERVICE
    let client = reqwest::Client::new();
    let julia_response = client
        .post("http://127.0.0.1:8081/score") 
        .json(&json!({ "title": payload.title }))
        .send()
        .await
        .expect("Failed to communicate with the Julia service at http://127.00.1:8081"); 

    let score_data = julia_response
        .json::<ScoreResponse>()
        .await
        .expect("Failed to parse JSON response from Julia service"); 

    let score = score_data.score;
    // --- END JULIA INTEGRATION LOGIC ---

    // 2. SAVE TO THE DATABASE (including the new score)
    sqlx::query("INSERT INTO todos (title, done, score) VALUES ($1, FALSE, $2)")
        .bind(&payload.title)
        .bind(score) // Bind the score from Julia
        .execute(&pool)
        .await
        .expect("Failed to insert todo");
    
    // Return 201 Created on success
    StatusCode::CREATED
}

/// Handler to update an existing todo item by ID.
///
/// PUT /todos/:id
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> StatusCode {
    // NOTE: This handler could also call Julia to update the score if title changes, 
    // but for now, we leave the logic simple.
    sqlx::query("UPDATE todos SET title = $1, done = $2 WHERE id = $3")
        .bind(&payload.title)
        .bind(&payload.done)
        .bind(id)
        .execute(&pool)
        .await
        .expect("Failed to update todo");

    // Return 204 No Content on successful update
    StatusCode::NO_CONTENT
}

/// Handler to delete a todo item by ID.
///
/// DELETE /todos/:id
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> StatusCode {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .expect("Failed to delete todo");

    // Return 204 No Content on successful deletion
    StatusCode::NO_CONTENT
}

/// Creates the main application router with all defined endpoints.
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        // /todos route handles GET and POST
        .route("/todos", get(get_todos).post(create_todo))
        // /todos/:id handles PUT (Update) and DELETE
        .route("/todos/:id", put(update_todo).delete(delete_todo))
        .with_state(pool)
}