use crate::modules::interfaces::document::WidgetNode;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

pub(super) struct WorldBossPreview;

impl WorldBossPreview {
    pub(super) fn rows(
        node: &WidgetNode,
        fields: &HashMap<String, Value>,
    ) -> Vec<HashMap<String, Value>> {
        node.binding
            .as_ref()
            .into_iter()
            .flat_map(|binding| &binding.boss_ids)
            .map(|id| {
                HashMap::from([
                    ("row.id".into(), json!(id.to_string())),
                    (
                        "row.name".into(),
                        fields
                            .get(&format!("worldBoss.{id}.name"))
                            .cloned()
                            .unwrap_or_else(|| json!(format!("#{id}"))),
                    ),
                    ("row.status".into(), json!("waiting")),
                    ("row.timer".into(), Value::Null),
                    ("row.remaining".into(), Value::Null),
                    ("row.observedAge".into(), Value::Null),
                ])
            })
            .collect()
    }
}
