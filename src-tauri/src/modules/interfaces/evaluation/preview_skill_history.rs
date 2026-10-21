use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewVisual;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

pub(super) struct PreviewSkillHistory;

impl PreviewSkillHistory {
    const INTERVAL_SECONDS: f64 = 1.8;
    const LIMIT: usize = 128;

    pub(super) fn rows(
        node: &WidgetNode,
        fields: &HashMap<String, Value>,
    ) -> Option<Vec<HashMap<String, Value>>> {
        let elapsed = fields.get("preview.elapsed")?.as_f64()?;

        if fields.get("preview.combat").and_then(Value::as_bool) != Some(true) {
            return Some(Vec::new());
        }

        let last = (elapsed / Self::INTERVAL_SECONDS).floor() as usize;

        Some(
            (0..=last)
                .rev()
                .take(Self::LIMIT)
                .filter_map(|sequence| {
                    let age = elapsed - sequence as f64 * Self::INTERVAL_SECONDS;

                    (age < node.group.span_seconds).then(|| {
                        HashMap::from([
                            ("row.id".into(), json!(sequence.to_string())),
                            ("row.name".into(), json!(node.name)),
                            ("row.remaining".into(), json!(node.group.span_seconds - age)),
                            (
                                "row.icon".into(),
                                fields
                                    .get(&format!("preview.icon.{}", sequence % 8))
                                    .cloned()
                                    .unwrap_or(Value::Null),
                            ),
                        ])
                    })
                })
                .collect(),
        )
    }

    pub(super) fn apply(
        node: &WidgetNode,
        row: &HashMap<String, Value>,
        visual: &mut PreviewVisual,
    ) {
        let remaining = row.get("row.remaining").and_then(Value::as_f64);

        visual.remaining = remaining;
        visual.duration = Some(node.group.span_seconds);
        visual.active = Some(true);

        if let Some(icon) = row.get("row.icon").and_then(Value::as_str) {
            visual.appearance.icon_path = Some(icon.into());
        }

        if let Some(remaining) = remaining {
            visual.appearance.opacity *= remaining.clamp(0.0, 1.0);
        }
    }
}
