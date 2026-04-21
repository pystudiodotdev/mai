//! Shared test utilities and helpers.

use sqlx::PgPool;

/// Create a test database pool from `DATABASE_URL`.
///
/// Tests that need a real database connection should call this helper. Make
/// sure a test-specific PostgreSQL database is available and that
/// `DATABASE_URL` is set (e.g. via `.env`).
pub async fn test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration tests");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("Failed to connect to test database")
}
