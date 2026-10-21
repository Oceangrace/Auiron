use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::rules::ComparisonOperator;
use crate::modules::interfaces::rules::ConditionMode;
use crate::modules::interfaces::rules::LoadState;
use crate::modules::interfaces::rules::TriggerRule;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) struct RuleEvaluator;

impl RuleEvaluator {
    pub(crate) fn combine(any: bool, values: &[Option<bool>]) -> Option<bool> {
        if values.contains(&Some(any)) {
            return Some(any);
        }

        if values.contains(&None) {
            return None;
        }

        Some(!any)
    }

    pub(crate) fn compare(
        left: Option<&Value>,
        operator: ComparisonOperator,
        right: &str,
    ) -> Option<bool> {
        let left = left.filter(|value| !value.is_null())?;

        if let Some(number) = left.as_f64() {
            let right: f64 = right.parse().ok()?;

            if !right.is_finite() {
                return None;
            }

            return Some(match operator {
                ComparisonOperator::Equal => number == right,
                ComparisonOperator::NotEqual => number != right,
                ComparisonOperator::Less => number < right,
                ComparisonOperator::LessEqual => number <= right,
                ComparisonOperator::Greater => number > right,
                ComparisonOperator::GreaterEqual => number >= right,
            });
        }

        let same = if let Some(flag) = left.as_bool() {
            flag == right.parse::<bool>().ok()?
        } else {
            left.as_str()? == right
        };

        match operator {
            ComparisonOperator::Equal => Some(same),
            ComparisonOperator::NotEqual => Some(!same),
            ComparisonOperator::Less
            | ComparisonOperator::LessEqual
            | ComparisonOperator::Greater
            | ComparisonOperator::GreaterEqual => None,
        }
    }

    pub(crate) fn active(
        node: &WidgetNode,
        fields: &HashMap<String, Value>,
        manual: Option<bool>,
    ) -> Option<bool> {
        if let Some(rules) = &node.rules {
            let values: Vec<Option<bool>> = rules
                .rules
                .iter()
                .filter(|rule| !rule.observe())
                .map(|rule| match rule {
                    TriggerRule::Value { trigger, .. } => Self::compare(
                        fields.get(&trigger.path),
                        trigger.operator.into(),
                        &trigger.value,
                    ),
                    _ => fields
                        .get(&format!("rule.{}.active", rule.id()))
                        .map_or(manual, Value::as_bool),
                })
                .collect();

            let group = Self::combine(rules.mode == ConditionMode::Any, &values);

            return if node.trigger.is_some() {
                Self::combine(false, &[group, manual])
            } else {
                group
            };
        }

        manual.or(if node.trigger.is_none() {
            Some(true)
        } else {
            None
        })
    }

    pub(crate) fn loaded(node: &WidgetNode, fields: &HashMap<String, Value>) -> Option<bool> {
        let mut values = vec![super::load_location::LoadLocation::matches(
            &node.load, fields,
        )];

        if node.load.in_group != LoadState::Any {
            let grouped = fields
                .get("group.size")
                .and_then(Value::as_f64)
                .map(|size| size > 1.0)
                .or_else(|| fields.get("group.active").and_then(Value::as_bool));

            values.push(grouped.map(|value| value == (node.load.in_group == LoadState::Yes)));
        }

        if node.load.in_combat != LoadState::Any {
            values.push(
                fields
                    .get("combat.inCombat")
                    .and_then(Value::as_bool)
                    .map(|value| value == (node.load.in_combat == LoadState::Yes)),
            );
        }

        if !node.load.player_class.is_empty() {
            values.push(
                fields
                    .get("player.class")
                    .and_then(Value::as_str)
                    .map(|value| value.eq_ignore_ascii_case(&node.load.player_class)),
            );
        }

        if !node.load.zone.is_empty() {
            values.push(
                fields
                    .get("zone.name")
                    .and_then(Value::as_str)
                    .map(|value| {
                        value
                            .to_lowercase()
                            .contains(&node.load.zone.to_lowercase())
                    }),
            );
        }

        Self::combine(false, &values)
    }
}
