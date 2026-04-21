//! Authentication server functions.
//!
//! Provides registration, login, and session-retrieval endpoints exposed as
//! Leptos server functions so they can be called transparently from components.

use crate::models::user::{CreateUser, UserPublic};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Result wrapper returned by auth endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub success: bool,
    pub message: String,
    pub user: Option<UserPublic>,
}

/// Register a new customer account.
#[server(RegisterUser, "/api")]
pub async fn register_user(input: CreateUser) -> Result<AuthResult, ServerFnError> {
    use crate::models::user::db as user_db;

    let pool = expect_context::<sqlx::PgPool>();

    // Check for existing email
    if let Some(_existing) = user_db::find_by_email(&pool, &input.email).await? {
        return Ok(AuthResult {
            success: false,
            message: "Email already registered.".into(),
            user: None,
        });
    }

    // Hash the password
    let password_hash =
        bcrypt::hash(&input.password, bcrypt::DEFAULT_COST).map_err(|e| {
            ServerFnError::new(e.to_string())
        })?;

    let user = user_db::create_user(&pool, &input.email, &input.username, &password_hash).await?;

    Ok(AuthResult {
        success: true,
        message: "Registration successful.".into(),
        user: Some(user.into()),
    })
}

/// Authenticate with email and password.
#[server(LoginUser, "/api")]
pub async fn login_user(email: String, password: String) -> Result<AuthResult, ServerFnError> {
    use crate::models::user::db as user_db;

    let pool = expect_context::<sqlx::PgPool>();

    let user = match user_db::find_by_email(&pool, &email).await? {
        Some(u) => u,
        None => {
            return Ok(AuthResult {
                success: false,
                message: "Invalid credentials.".into(),
                user: None,
            })
        }
    };

    let valid = bcrypt::verify(&password, &user.password_hash).map_err(|e| {
        ServerFnError::new(e.to_string())
    })?;

    if !valid {
        return Ok(AuthResult {
            success: false,
            message: "Invalid credentials.".into(),
            user: None,
        });
    }

    // TODO: Set session cookie / JWT here in a production implementation.

    Ok(AuthResult {
        success: true,
        message: "Login successful.".into(),
        user: Some(user.into()),
    })
}

/// Retrieve the currently authenticated user (placeholder).
///
/// In a production system this would inspect the session cookie / JWT to
/// identify the caller. For now it returns `None`.
#[server(GetCurrentUser, "/api")]
pub async fn get_current_user() -> Result<Option<UserPublic>, ServerFnError> {
    // TODO: Implement session-based lookup.
    Ok(None)
}
