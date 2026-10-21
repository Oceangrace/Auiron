use super::HealingPrediction;
use super::PreviewVisual;
use crate::core::game::HealthSnapshot;
use crate::modules::interfaces::appearance::HealingPredictionTarget;
use serde_json::Value;
use std::collections::HashMap;

pub(super) struct PreviewHealing;

impl PreviewHealing {
    pub(super) fn apply(visual: &mut PreviewVisual, fields: &HashMap<String, Value>) {
        if !visual.simulating || !fields.contains_key("preview.elapsed") {
            return;
        }

        let prefix = match visual.style.healing_prediction {
            Some(HealingPredictionTarget::Player) => "player",
            Some(HealingPredictionTarget::Target) => "target",
            Some(HealingPredictionTarget::Entity) => "entity",
            None => return,
        };

        let current = Self::resource(fields, &format!("{prefix}.hp.value"));

        let maximum = Self::resource(fields, &format!("{prefix}.hp.max"));

        let amount = fields
            .get(&format!("{prefix}.hp.healing"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0);

        visual.healing_prediction =
            HealingPrediction::new(&HealthSnapshot::new(current, maximum), amount);
    }

    fn resource(fields: &HashMap<String, Value>, key: &str) -> Option<u32> {
        fields
            .get(key)
            .and_then(Value::as_u64)
            .and_then(|value| u32::try_from(value).ok())
    }
}
