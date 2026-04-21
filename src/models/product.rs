//! Product domain model and database operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A product listed in the mai catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub sku: Option<String>,
    pub stock_qty: i32,
    pub category_id: Option<Uuid>,
    pub is_active: bool,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payload for creating or updating a product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPayload {
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub sku: Option<String>,
    pub stock_qty: i32,
    pub category_id: Option<Uuid>,
    pub is_active: bool,
    pub image_url: Option<String>,
}

// ── Database helpers (server-only) ──────────────────────────────────────────

#[cfg(feature = "ssr")]
pub mod db {
    use super::*;
    use sqlx::PgPool;

    /// Insert a new product.
    pub async fn create_product(
        pool: &PgPool,
        p: &ProductPayload,
    ) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "INSERT INTO products (name, description, price_cents, currency, sku,
                                   stock_qty, category_id, is_active, image_url)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, name, description, price_cents, currency, sku,
                       stock_qty, category_id, is_active, image_url,
                       created_at, updated_at",
        )
        .bind(&p.name)
        .bind(&p.description)
        .bind(p.price_cents)
        .bind(&p.currency)
        .bind(&p.sku)
        .bind(p.stock_qty)
        .bind(p.category_id)
        .bind(p.is_active)
        .bind(&p.image_url)
        .fetch_one(pool)
        .await
    }

    /// List all active products (storefront view).
    pub async fn list_active_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, description, price_cents, currency, sku,
                    stock_qty, category_id, is_active, image_url,
                    created_at, updated_at
             FROM products WHERE is_active = true
             ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// List all products (admin view — includes inactive).
    pub async fn list_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, description, price_cents, currency, sku,
                    stock_qty, category_id, is_active, image_url,
                    created_at, updated_at
             FROM products ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Find a product by ID.
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Product>, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "SELECT id, name, description, price_cents, currency, sku,
                    stock_qty, category_id, is_active, image_url,
                    created_at, updated_at
             FROM products WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// Update an existing product.
    pub async fn update_product(
        pool: &PgPool,
        id: Uuid,
        p: &ProductPayload,
    ) -> Result<Product, sqlx::Error> {
        sqlx::query_as::<_, Product>(
            "UPDATE products SET
                 name = $2, description = $3, price_cents = $4, currency = $5,
                 sku = $6, stock_qty = $7, category_id = $8, is_active = $9,
                 image_url = $10, updated_at = now()
             WHERE id = $1
             RETURNING id, name, description, price_cents, currency, sku,
                       stock_qty, category_id, is_active, image_url,
                       created_at, updated_at",
        )
        .bind(id)
        .bind(&p.name)
        .bind(&p.description)
        .bind(p.price_cents)
        .bind(&p.currency)
        .bind(&p.sku)
        .bind(p.stock_qty)
        .bind(p.category_id)
        .bind(p.is_active)
        .bind(&p.image_url)
        .fetch_one(pool)
        .await
    }

    /// Delete a product by ID.
    pub async fn delete_product(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
