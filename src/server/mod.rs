//! Server-side logic for the mai application.
//!
//! This module contains Leptos server functions, the database connection pool,
//! and all back-end business logic that runs exclusively under the `ssr` feature.

pub mod admin;
pub mod auth;
pub mod products;

// ── Database pool (server-only) ─────────────────────────────────────────────

#[cfg(feature = "ssr")]
pub mod db {
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;

    /// Create a PostgreSQL connection pool using the `DATABASE_URL` env var.
    ///
    /// Call this once at application startup and store the pool in Axum state.
    pub async fn create_pool() -> PgPool {
        dotenvy::dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Failed to create database pool")
    }

    /// Run pending sqlx migrations against the database.
    pub async fn run_migrations(pool: &PgPool) {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .expect("Failed to run database migrations");
    }
}
