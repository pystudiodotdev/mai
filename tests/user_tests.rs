//! Integration tests for user-related database operations.
//!
//! These tests require a running PostgreSQL instance with the schema applied.
//! Run: `sqlx migrate run` before executing the test suite.

mod common;

use mai::models::user::{UserRole, UserPublic};

/// Verify that `UserRole` round-trips through its string representation.
#[test]
fn user_role_round_trip() {
    assert_eq!(UserRole::from_str(UserRole::Admin.as_str()), UserRole::Admin);
    assert_eq!(
        UserRole::from_str(UserRole::Customer.as_str()),
        UserRole::Customer
    );
}

/// Unknown role strings should default to `Customer`.
#[test]
fn unknown_role_defaults_to_customer() {
    assert_eq!(UserRole::from_str("superadmin"), UserRole::Customer);
    assert_eq!(UserRole::from_str(""), UserRole::Customer);
}

/// `UserPublic` conversion must not include the password hash.
#[test]
fn user_public_excludes_password_hash() {
    use chrono::Utc;
    use uuid::Uuid;

    let user = mai::models::user::User {
        id: Uuid::new_v4(),
        email: "test@example.com".into(),
        username: "tester".into(),
        password_hash: "secret_hash".into(),
        role: "customer".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let public: UserPublic = user.into();
    // `UserPublic` has no `password_hash` field — this test validates
    // that the conversion compiles and the public fields are correct.
    assert_eq!(public.email, "test@example.com");
    assert_eq!(public.username, "tester");
}

/// Integration test: create and retrieve a user from the database.
///
/// This test is ignored by default because it requires a running Postgres
/// instance. Run with `cargo test --features ssr -- --ignored` after setting up the DB.
#[cfg(feature = "ssr")]
#[tokio::test]
#[ignore]
async fn create_and_find_user() {
    use mai::models::user::db as user_db;

    let pool = common::test_pool().await;

    let email = format!("test+{}@mai.dev", uuid::Uuid::new_v4());
    let user = user_db::create_user(&pool, &email, "testuser", "$2b$12$dummyhash")
        .await
        .expect("Failed to create user");

    assert_eq!(user.email, email);

    let found = user_db::find_by_email(&pool, &email)
        .await
        .expect("Query failed")
        .expect("User not found");

    assert_eq!(found.id, user.id);
}
