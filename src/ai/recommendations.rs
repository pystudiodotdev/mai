//! Product recommendation engine — placeholder.
//!
//! Future implementations could use collaborative filtering, content-based
//! approaches, or call out to an external ML service.

use uuid::Uuid;

/// Trait for product recommendation engines.
pub trait RecommendationEngine: Send + Sync {
    /// Given a product ID, return a list of recommended product IDs.
    ///
    /// The default implementation returns an empty list (no recommendations).
    fn recommend_for_product(&self, _product_id: Uuid) -> Vec<Uuid> {
        Vec::new()
    }

    /// Given a user ID, return personalised product recommendations.
    ///
    /// The default implementation returns an empty list.
    fn recommend_for_user(&self, _user_id: Uuid) -> Vec<Uuid> {
        Vec::new()
    }
}

/// No-op implementation — always returns empty recommendations.
pub struct NoOpRecommendations;

impl RecommendationEngine for NoOpRecommendations {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_op_returns_empty() {
        let engine = NoOpRecommendations;
        let recs = engine.recommend_for_product(Uuid::new_v4());
        assert!(recs.is_empty());
    }
}
