use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

/// Establishes and returns a connection pool to the PostgreSQL database.
///
/// Panics if the `DATABASE_URL` environment variable is not set or if the connection fails.
pub async fn connect() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    
    // Configure and connect the pool.
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("❌ Failed to connect to the database")
}