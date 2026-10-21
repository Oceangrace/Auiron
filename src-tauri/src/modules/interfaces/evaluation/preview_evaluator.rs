use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::FormulaDefinition;
use crate::modules::interfaces::evaluation::FormulaEngine;
use crate::modules::interfaces::evaluation::PreviewContext;
use crate::modules::interfaces::evaluation::PreviewFields;
use crate::modules::interfaces::evaluation::PreviewInstance;
use crate::modules::interfaces::evaluation::PreviewRows;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::evaluation::WidgetVisualResolver;
use crate::modules::interfaces::layout::Direction;
use crate::modules::interfaces::layout::GroupDataSource;
use crate::modules::interfaces::rules::TriggerRule;
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;

pub(crate) struct PreviewEvaluator;

impl PreviewEvaluator {
    pub(crate) fn evaluate(
        nodes: &[WidgetNode],
        context: &PreviewContext,
        formulas: &[FormulaDefinition],
    ) -> Result<HashMap<String, PreviewVisual>, String> {
        let mut fields = context.fields.clone();

        for (key, value) in [
            ("t1.remaining", context.remaining.map(|v| json!(v * 1000.0))),
            ("t1.stacks", context.stacks.map(|v| json!(v))),
            ("progress", context.progress.map(|v| json!(v))),
        ] {
            if let Some(value) = value {
                fields.insert(key.into(), value);
            }
        }

        let fields = FormulaEngine::fields(formulas, &fields);

        let mut result = HashMap::new();

        Self::nodes(nodes, context, formulas, &fields, &mut result)?;

        Ok(result)
    }

    fn nodes(
        nodes: &[WidgetNode],
        context: &PreviewContext,
        formulas: &[FormulaDefinition],
        fields: &HashMap<String, Value>,
        result: &mut HashMap<String, PreviewVisual>,
    ) -> Result<(), String> {
        for node in nodes {
            let mut data = fields.clone();

            if let Some(widget_fields) = context.widgets.get(&node.id) {
                data.extend(widget_fields.clone());
            }

            let selected = PreviewFields::resolve(node, &data, formulas);

            let mut visual = WidgetVisualResolver::resolve(node, context, &selected)?;

            if let Some(instances) = context.instances.get(&node.id) {
                if context.simulating
                    && instances.is_empty()
                    && node
                        .binding
                        .as_ref()
                        .is_none_or(|binding| binding.source != GroupDataSource::Effects)
                {
                    visual.visible = Some(false);
                }

                let limit = if node.group.limit > 0.0 {
                    node.group.limit as usize
                } else {
                    instances.len()
                };

                for (index, instance) in instances.iter().take(limit).enumerate() {
                    let mut data = data.clone();

                    data.extend(instance.clone());

                    let data = PreviewFields::resolve(node, &data, formulas);

                    let mut item = WidgetVisualResolver::resolve(node, context, &data)?;

                    let direction = node
                        .rules
                        .as_ref()
                        .and_then(|group| {
                            group.rules.iter().find_map(|rule| match rule {
                                TriggerRule::Effect { trigger, .. } if trigger.clones => {
                                    Some(trigger.clone_direction)
                                }

                                _ => None,
                            })
                        })
                        .unwrap_or_default();

                    let offset = index as f64;

                    match direction {
                        Direction::Left => {
                            item.offset_x = -offset * (node.layout.width * node.scale() + 4.0)
                        }
                        Direction::Up => {
                            item.offset_y = -offset * (node.layout.height * node.scale() + 4.0)
                        }
                        Direction::Down => {
                            item.offset_y = offset * (node.layout.height * node.scale() + 4.0)
                        }
                        Direction::Right => {
                            item.offset_x = offset * (node.layout.width * node.scale() + 4.0)
                        }
                    }

                    visual.instances.push(PreviewInstance {
                        key: index.to_string(),
                        visual: Box::new(item),
                    });
                }
            }

            result.insert(node.id.clone(), visual);
            Self::nodes(&node.children, context, formulas, fields, result)?;

            if node
                .binding
                .as_ref()
                .is_some_and(|binding| binding.source != GroupDataSource::Effects)
                && (context.simulating
                    || !context.rows.is_empty()
                    || node
                        .binding
                        .as_ref()
                        .is_some_and(|binding| binding.source == GroupDataSource::WorldBosses))
            {
                PreviewRows::populate(node, context, fields, result, |child, data| {
                    let mut row = HashMap::new();

                    Self::nodes(
                        std::slice::from_ref(child),
                        context,
                        formulas,
                        data,
                        &mut row,
                    )?;

                    Ok(row)
                })?;
            }
        }

        Ok(())
    }
}
