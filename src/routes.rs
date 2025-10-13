use axum::{
    extract::{State, Path},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

/// The Todo model struct, used for sending and receiving todo items.
#[derive(Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
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
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch todos");

    Json(todos)
}

/// Handler to create a new todo item.
///
/// POST /todos
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<NewTodo>,
) -> StatusCode {
    sqlx::query("INSERT INTO todos (title, done) VALUES ($1, FALSE)")
        .bind(&payload.title)
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