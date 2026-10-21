use crate::core::game::EntityId;
use serde_json::Value;
use std::collections::HashMap;

pub(super) struct LiveRow {
    key: String,
    entity_id: Option<EntityId>,
    fields: HashMap<String, Value>,
}

impl LiveRow {
    pub(super) fn new(
        key: String,
        entity_id: Option<EntityId>,
        fields: HashMap<String, Value>,
    ) -> Self {
        Self {
            key,
            entity_id,
            fields,
        }
    }

    pub(super) fn percentages(&mut self, total: Option<f64>, leader: Option<f64>) {
        let Some(value) = self.fields.get("row.value").and_then(Value::as_f64) else {
            return;
        };

        if let Some(total) = total.filter(|total| *total > 0.0) {
            self.fields.insert(
                "row.percent".into(),
                serde_json::json!(value / total * 100.0),
            );
        }

        if let Some(leader) = leader.filter(|leader| *leader > 0.0) {
            self.fields.insert(
                "row.leaderPercent".into(),
                serde_json::json!(value / leader * 100.0),
            );
        }
    }

    pub(super) fn key(&self) -> &str {
        &self.key
    }

    pub(super) fn entity_id(&self) -> Option<EntityId> {
        self.entity_id
    }

    pub(super) fn fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }
}
