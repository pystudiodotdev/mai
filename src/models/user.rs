//! User domain model and database operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user's role within the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Customer,
    Admin,
}

impl UserRole {
    /// Convert the role to its database string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Customer => "customer",
            UserRole::Admin => "admin",
        }
    }

    /// Parse a role from its database string representation.
    pub fn from_str(s: &str) -> Self {
        match s {
            "admin" => UserRole::Admin,
            _ => UserRole::Customer,
        }
    }
}

/// A registered user in the mai system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payload used when creating a new user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// Lightweight user info suitable for client-side use (no password hash).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            username: u.username,
            role: u.role,
            created_at: u.created_at,
        }
    }
}

// ── Database helpers (server-only) ──────────────────────────────────────────

#[cfg(feature = "ssr")]
pub mod db {
    use super::*;
    use sqlx::PgPool;

    /// Insert a new user and return the created row.
    pub async fn create_user(
        pool: &PgPool,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (email, username, password_hash)
             VALUES ($1, $2, $3)
             RETURNING id, email, username, password_hash, role, created_at, updated_at",
        )
        .bind(email)
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    /// Look up a user by email.
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, username, password_hash, role, created_at, updated_at
             FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    /// Look up a user by ID.
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, username, password_hash, role, created_at, updated_at
             FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// List all users (admin use).
    pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, username, password_hash, role, created_at, updated_at
             FROM users ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }
}
