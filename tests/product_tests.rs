//! Integration tests for product-related database operations.
//!
//! These tests require a running PostgreSQL instance with the schema applied.
//! Run: `sqlx migrate run` before executing the test suite.

mod common;

use mai::models::product::ProductPayload;

/// Verify that `ProductPayload` can be serialised and deserialised.
#[test]
fn product_payload_serde_round_trip() {
    let payload = ProductPayload {
        name: "Widget".into(),
        description: Some("A fine widget".into()),
        price_cents: 1999,
        currency: "USD".into(),
        sku: Some("WDG-001".into()),
        stock_qty: 50,
        category_id: None,
        is_active: true,
        image_url: None,
    };

    let json = serde_json::to_string(&payload).expect("serialize");
    let deserialized: ProductPayload = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(deserialized.name, "Widget");
    assert_eq!(deserialized.price_cents, 1999);
    assert_eq!(deserialized.stock_qty, 50);
}

/// Integration test: create, retrieve, and delete a product.
///
/// Ignored by default — requires a running Postgres instance.
/// Run with `cargo test --features ssr -- --ignored` after setting up the DB.
#[cfg(feature = "ssr")]
#[tokio::test]
#[ignore]
async fn product_crud_lifecycle() {
    use mai::models::product::db as product_db;

    let pool = common::test_pool().await;

    let payload = ProductPayload {
        name: format!("Test Product {}", uuid::Uuid::new_v4()),
        description: Some("Integration test product".into()),
        price_cents: 4999,
        currency: "USD".into(),
        sku: Some(format!("TST-{}", &uuid::Uuid::new_v4().to_string()[..8])),
        stock_qty: 10,
        category_id: None,
        is_active: true,
        image_url: None,
    };

    // Create
    let product = product_db::create_product(&pool, &payload)
        .await
        .expect("Failed to create product");
    assert_eq!(product.name, payload.name);

    // Read
    let found = product_db::find_by_id(&pool, product.id)
        .await
        .expect("Query failed")
        .expect("Product not found");
    assert_eq!(found.id, product.id);

    // Delete
    let deleted = product_db::delete_product(&pool, product.id)
        .await
        .expect("Delete failed");
    assert!(deleted);

    // Verify deletion
    let gone = product_db::find_by_id(&pool, product.id)
        .await
        .expect("Query failed");
    assert!(gone.is_none());
}
