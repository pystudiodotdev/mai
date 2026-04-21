//! AI module — placeholder for future intelligent features.
//!
//! This module serves as the integration point for AI-powered capabilities
//! such as product recommendations, demand forecasting, customer segmentation,
//! and automated pricing. Each sub-module defines a trait-based interface so
//! that concrete implementations can be swapped in as they are developed.

pub mod forecasting;
pub mod recommendations;

/// A placeholder trait representing an AI service provider.
///
/// Implementations might call an external ML model API, a local ONNX runtime,
/// or a simple heuristic engine during early development.
pub trait AiProvider: Send + Sync {
    /// Human-readable name of the provider (e.g. "OpenAI", "Local-ONNX").
    fn name(&self) -> &str;
}

/// A no-op provider used as the default until a real implementation is wired in.
pub struct NoOpProvider;

impl AiProvider for NoOpProvider {
    fn name(&self) -> &str {
        "no-op"
    }
}
