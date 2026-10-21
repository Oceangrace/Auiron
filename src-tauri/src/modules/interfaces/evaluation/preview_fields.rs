use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::FormulaDefinition;
use crate::modules::interfaces::evaluation::FormulaEngine;
use crate::modules::interfaces::rules::ObservationKind;
use crate::modules::interfaces::rules::RuleKind;
use crate::modules::interfaces::rules::TriggerRule;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

pub(crate) struct PreviewFields;

impl PreviewFields {
    pub(crate) fn resolve(
        node: &WidgetNode,
        fields: &HashMap<String, Value>,
        formulas: &[FormulaDefinition],
    ) -> HashMap<String, Value> {
        let mut data = fields.clone();

        let selected_rule = node
            .rules
            .as_ref()
            .and_then(|rules| rules.data_rule_id.as_deref())
            .filter(|id| {
                node.trigger
                    .as_ref()
                    .is_none_or(|trigger| trigger.observation != ObservationKind::SkillState)
                    || fields
                        .get(&format!("rule.{id}.active"))
                        .and_then(Value::as_bool)
                        == Some(true)
            });

        if let Some(id) = selected_rule {
            let prefix = format!("rule.{id}.");

            data.retain(|key, _| !key.starts_with("t1."));

            for (key, value) in fields {
                if let Some(key) = key.strip_prefix(&prefix) {
                    data.insert(format!("t1.{key}"), value.clone());
                }
            }
        }

        let event = node
            .rules
            .as_ref()
            .and_then(|group| {
                group
                    .rules
                    .iter()
                    .find(|rule| Some(rule.id()) == selected_rule)
            })
            .is_some_and(|rule| rule.kind() == RuleKind::Event);

        if event {
            let selected: Vec<_> = data
                .iter()
                .filter_map(|(key, value)| {
                    key.strip_prefix("t1.")
                        .map(|key| (format!("event.{key}"), value.clone()))
                })
                .collect();

            data.extend(selected);

            if let Some(value) = data
                .get("t1.countdown")
                .or_else(|| data.get("event.countdown"))
                .or_else(|| data.get("event.remaining"))
                .cloned()
            {
                data.insert("t1.remaining".into(), value);
            }

            if let Some(value) = data.get("event.duration").cloned() {
                data.insert("t1.duration".into(), value);
            }
        }

        Self::boss_names(node, selected_rule, &mut data);
        Self::boss_timer_precision(node, selected_rule, &mut data);

        FormulaEngine::fields(formulas, &data)
    }

    fn boss_timer_precision(
        node: &WidgetNode,
        selected: Option<&str>,
        fields: &mut HashMap<String, Value>,
    ) {
        for rule in node
            .rules
            .iter()
            .flat_map(|group| &group.rules)
            .filter(|rule| rule.kind() == RuleKind::Boss)
        {
            Self::whole_seconds(&format!("rule.{}", rule.id()), fields);

            if selected == Some(rule.id()) {
                Self::whole_seconds("t1", fields);
            }
        }
    }

    fn whole_seconds(prefix: &str, fields: &mut HashMap<String, Value>) {
        // Formatting must not round the numeric values used by conditions and progress bars.
        for name in ["remaining", "duration", "countdown"] {
            fields.insert(format!("wholeSeconds.{prefix}.{name}"), json!(true));
        }
    }

    fn boss_names(node: &WidgetNode, selected: Option<&str>, fields: &mut HashMap<String, Value>) {
        if !fields.contains_key("preview.elapsed") {
            return;
        }

        for rule in node.rules.iter().flat_map(|group| &group.rules) {
            if let TriggerRule::Boss { id, trigger, .. } = rule
                && let Some(name) = fields
                    .get(&format!("worldBoss.{}.name", trigger.boss_id))
                    .cloned()
            {
                fields.insert(format!("rule.{id}.name"), name.clone());

                if selected == Some(id.as_str()) {
                    fields.insert("t1.name".into(), name);
                }
            }
        }
    }
}
