use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

mod db;
mod routes; // Contains all Axum route handlers and the router logic

#[tokio::main]
async fn main() {
    // Load environment variables from a .env file.
    dotenvy::dotenv().ok();
    // Initialize tracing (logging) for the application.
    tracing_subscriber::fmt::init();

    // Connect to the PostgreSQL database.
    let pool = db::connect().await;
    // Create the Axum application router with all defined routes.
    let app = routes::create_router(pool);

    // --- CORS Setup (Essential for frontend communication) ---
    // Create a new CORS layer that allows requests from any origin, using any method, and any header.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Apply the CORS middleware to the application.
    let app = app.layer(cors);

    // Define the server address and port.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Start the server listener.
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}