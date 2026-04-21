//! Demand forecasting engine — placeholder.
//!
//! Future implementations could use time-series analysis, regression models,
//! or external forecasting APIs to predict product demand.

use uuid::Uuid;

/// A forecast result for a single product.
#[derive(Debug, Clone)]
pub struct DemandForecast {
    pub product_id: Uuid,
    /// Predicted number of units to be sold in the forecast period.
    pub predicted_units: u32,
    /// Confidence level of the prediction (0.0–1.0).
    pub confidence: f64,
}

/// Trait for demand forecasting engines.
pub trait ForecastingEngine: Send + Sync {
    /// Forecast demand for a given product over the default time horizon.
    ///
    /// The default implementation returns a zero-confidence forecast.
    fn forecast_demand(&self, product_id: Uuid) -> DemandForecast {
        DemandForecast {
            product_id,
            predicted_units: 0,
            confidence: 0.0,
        }
    }
}

/// No-op implementation — always returns zero-confidence forecasts.
pub struct NoOpForecasting;

impl ForecastingEngine for NoOpForecasting {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_op_forecast_is_zero_confidence() {
        let engine = NoOpForecasting;
        let forecast = engine.forecast_demand(Uuid::new_v4());
        assert_eq!(forecast.predicted_units, 0);
        assert!((forecast.confidence - 0.0).abs() < f64::EPSILON);
    }
}
