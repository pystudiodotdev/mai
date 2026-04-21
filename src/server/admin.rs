//! Admin-specific server functions.
//!
//! These endpoints are intended for use in the protected admin dashboard and
//! should only be accessible to users with the `admin` role.

use crate::models::category::{Category, CategoryPayload};
use crate::models::product::Product;
use crate::models::user::UserPublic;
use leptos::prelude::*;

/// List all users (admin).
#[server(AdminListUsers, "/api")]
pub async fn admin_list_users() -> Result<Vec<UserPublic>, ServerFnError> {
    use crate::models::user::db as user_db;

    // TODO: Verify admin privileges before returning data.
    let pool = expect_context::<sqlx::PgPool>();
    let users: Vec<UserPublic> = user_db::list_users(&pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(users)
}

/// List all products including inactive ones (admin).
#[server(AdminListProducts, "/api")]
pub async fn admin_list_products() -> Result<Vec<Product>, ServerFnError> {
    use crate::models::product::db as product_db;

    // TODO: Verify admin privileges before returning data.
    let pool = expect_context::<sqlx::PgPool>();
    let products = product_db::list_all_products(&pool).await?;
    Ok(products)
}

/// List all categories (admin).
#[server(AdminListCategories, "/api")]
pub async fn admin_list_categories() -> Result<Vec<Category>, ServerFnError> {
    use crate::models::category::db as cat_db;

    let pool = expect_context::<sqlx::PgPool>();
    let categories = cat_db::list_categories(&pool).await?;
    Ok(categories)
}

/// Create a new category (admin).
#[server(AdminCreateCategory, "/api")]
pub async fn admin_create_category(payload: CategoryPayload) -> Result<Category, ServerFnError> {
    use crate::models::category::db as cat_db;

    // TODO: Verify admin privileges before mutating data.
    let pool = expect_context::<sqlx::PgPool>();
    let category = cat_db::create_category(&pool, &payload).await?;
    Ok(category)
}
