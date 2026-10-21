use crate::modules::interfaces::document::WidgetKind;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewContext;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::evaluation::RuleEvaluator;
use crate::modules::interfaces::evaluation::VisualChanges;
use crate::modules::interfaces::rules::ConditionCheck;
use crate::modules::interfaces::rules::ConditionField;
use crate::modules::interfaces::rules::ConditionMode;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

pub(crate) struct ConditionEvaluator;

impl ConditionEvaluator {
    pub(crate) fn apply(
        node: &WidgetNode,
        visual: &mut PreviewVisual,
        context: &PreviewContext,
        fields: &HashMap<String, Value>,
    ) -> Result<(), String> {
        for condition in &node.conditions {
            if !condition.enabled {
                continue;
            }

            let checks: Vec<Option<bool>> = condition
                .checks
                .iter()
                .map(|check| Self::check(check, node, visual, context, fields))
                .collect();

            if RuleEvaluator::combine(condition.mode == ConditionMode::Any, &checks) == Some(true) {
                for change in &condition.changes {
                    VisualChanges::apply(visual, change)?;
                }
            }
        }

        Ok(())
    }

    fn check(
        check: &ConditionCheck,
        node: &WidgetNode,
        visual: &PreviewVisual,
        context: &PreviewContext,
        fields: &HashMap<String, Value>,
    ) -> Option<bool> {
        if check.field == ConditionField::Custom {
            return RuleEvaluator::compare(
                fields.get(check.path.as_deref()?),
                check.operator,
                check.text_value.as_deref()?,
            );
        }

        let value = match check.field {
            ConditionField::Active => visual.active.map(|value| json!(if value { 1 } else { 0 })),
            ConditionField::Remaining => visual.remaining.map(|value| json!(value)),
            ConditionField::Stacks => visual.stacks.map(|value| json!(value)),
            ConditionField::Progress => context
                .progress
                .or(if node.kind == WidgetKind::Bar && visual.known_progress {
                    Some(visual.style.bar_value)
                } else {
                    None
                })
                .map(|value| json!(value)),
            ConditionField::Custom => None,
        };

        RuleEvaluator::compare(value.as_ref(), check.operator, &check.value.to_string())
    }
}
