use crate::core::game::UnitSnapshot;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

pub(super) struct CastFields;

impl CastFields {
    pub(super) fn collect(
        fields: &mut HashMap<String, Value>,
        prefix: &str,
        unit: &UnitSnapshot,
        now_us: u64,
    ) {
        let Some(cast) = unit.cast() else {
            return;
        };

        let remaining = cast.remaining_ms(now_us);

        fields.insert(
            format!("{prefix}.cast.skillId"),
            json!(cast.skill_id().get()),
        );
        fields.insert(
            format!("{prefix}.cast.startedAt"),
            json!(cast.started_at_us() / 1000),
        );
        fields.insert(format!("{prefix}.cast.duration"), json!(cast.duration_ms()));
        fields.insert(format!("{prefix}.cast.remaining"), json!(remaining));
        fields.insert(
            format!("{prefix}.cast.percent"),
            json!(remaining / f64::from(cast.duration_ms()) * 100.0),
        );
        fields.insert(format!("{prefix}.cast.active"), json!(remaining > 0.0));

        if let Some(target) = cast.target_id() {
            fields.insert(
                format!("{prefix}.cast.targetId"),
                json!(target.get().to_string()),
            );
        }
    }
}
