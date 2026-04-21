//! Data models for the mai e-commerce system.
//!
//! Each sub-module defines the domain types and, when the `ssr` feature is
//! enabled, the corresponding database query helpers backed by [`sqlx`].

pub mod category;
pub mod product;
pub mod user;
