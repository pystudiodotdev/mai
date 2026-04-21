//! Category domain model and database operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A product category used to organise the catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payload for creating or updating a category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPayload {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

// ── Database helpers (server-only) ──────────────────────────────────────────

#[cfg(feature = "ssr")]
pub mod db {
    use super::*;
    use sqlx::PgPool;

    /// Insert a new category.
    pub async fn create_category(
        pool: &PgPool,
        payload: &CategoryPayload,
    ) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "INSERT INTO categories (name, description, parent_id)
             VALUES ($1, $2, $3)
             RETURNING id, name, description, parent_id, created_at, updated_at",
        )
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(payload.parent_id)
        .fetch_one(pool)
        .await
    }

    /// List all categories.
    pub async fn list_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "SELECT id, name, description, parent_id, created_at, updated_at
             FROM categories ORDER BY name",
        )
        .fetch_all(pool)
        .await
    }

    /// Find a category by ID.
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "SELECT id, name, description, parent_id, created_at, updated_at
             FROM categories WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }
}
