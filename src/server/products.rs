//! Product-related server functions.
//!
//! Exposes CRUD operations for products as Leptos server functions, making them
//! callable from both server-rendered and client-hydrated contexts.

use crate::models::product::{Product, ProductPayload};
use leptos::prelude::*;
use uuid::Uuid;

/// Fetch all active products (public storefront).
#[server(ListActiveProducts, "/api")]
pub async fn list_active_products() -> Result<Vec<Product>, ServerFnError> {
    use crate::models::product::db;

    let pool = expect_context::<sqlx::PgPool>();
    let products = db::list_active_products(&pool).await?;
    Ok(products)
}

/// Fetch a single product by ID (public storefront).
#[server(GetProduct, "/api")]
pub async fn get_product(id: Uuid) -> Result<Option<Product>, ServerFnError> {
    use crate::models::product::db;

    let pool = expect_context::<sqlx::PgPool>();
    let product = db::find_by_id(&pool, id).await?;
    Ok(product)
}

/// Create a new product (admin only).
#[server(CreateProduct, "/api")]
pub async fn create_product(payload: ProductPayload) -> Result<Product, ServerFnError> {
    use crate::models::product::db;

    // TODO: Verify that the caller has admin privileges.
    let pool = expect_context::<sqlx::PgPool>();
    let product = db::create_product(&pool, &payload).await?;
    Ok(product)
}

/// Update an existing product (admin only).
#[server(UpdateProduct, "/api")]
pub async fn update_product(id: Uuid, payload: ProductPayload) -> Result<Product, ServerFnError> {
    use crate::models::product::db;

    // TODO: Verify that the caller has admin privileges.
    let pool = expect_context::<sqlx::PgPool>();
    let product = db::update_product(&pool, id, &payload).await?;
    Ok(product)
}

/// Delete a product by ID (admin only).
#[server(DeleteProduct, "/api")]
pub async fn delete_product(id: Uuid) -> Result<bool, ServerFnError> {
    use crate::models::product::db;

    // TODO: Verify that the caller has admin privileges.
    let pool = expect_context::<sqlx::PgPool>();
    let deleted = db::delete_product(&pool, id).await?;
    Ok(deleted)
}
