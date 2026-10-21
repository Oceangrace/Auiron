use crate::core::game::HealthSnapshot;
use serde::Serialize;

/// Estimated remaining periodic healing, assuming no further damage or healing.
#[derive(Clone, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HealingPrediction {
    start_percent: f64,
    end_percent: f64,
    overheal_percent: f64,
    amount: f64,
    overheal: f64,
}

impl HealingPrediction {
    pub(super) fn new(health: &HealthSnapshot, amount: f64) -> Option<Self> {
        let current = f64::from(health.current()?);

        let maximum = f64::from(health.maximum()?);

        if maximum <= 0.0 || current <= 0.0 || !amount.is_finite() || amount <= 0.0 {
            return None;
        }

        let missing = (maximum - current).max(0.0);

        let effective = amount.min(missing);

        Some(Self {
            start_percent: (current / maximum * 100.0).clamp(0.0, 100.0),
            end_percent: ((current + effective) / maximum * 100.0).clamp(0.0, 100.0),
            overheal_percent: ((amount - effective).max(0.0) / maximum * 100.0).clamp(0.0, 100.0),
            amount: amount.round(),
            overheal: (amount - effective).max(0.0).round(),
        })
    }
}
