// main.rs
mod db;
mod routes;
mod error;
mod auth;
mod middleware;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use routes::protected_route;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool = db::connect().await;
    
    // CORS Layer definieren
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Router aufbauen
    let app = Router::new()
        // Auth routes (ungeschützt)
        .nest("/auth", auth::auth_routes(pool.clone()))
        // Protected sample route
        .route("/protected", axum::routing::get(protected_route))
        // Todo routes (geschützt mit AuthUser)
        .nest("/todos", routes::create_router(pool.clone()))
        .layer(cors);  // CORS als letztes Layer hinzufügen
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);
    println!("📋 Routes:");
    println!("   POST   /auth/login");
    println!("   GET    /protected");
    println!("   GET    /todos");
    println!("   POST   /todos");
    println!("   PUT    /todos/:id");
    println!("   DELETE /todos/:id");
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(), 
        app
    )
    .await
    .unwrap();
}