use super::preview_healing::PreviewHealing;
use crate::modules::interfaces::appearance::AnimationChange;
use crate::modules::interfaces::appearance::AnimationWhen;
use crate::modules::interfaces::appearance::ProgressSource;
use crate::modules::interfaces::document::WidgetKind;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::ConditionEvaluator;
use crate::modules::interfaces::evaluation::PreviewContext;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::evaluation::RuleEvaluator;
use crate::modules::interfaces::evaluation::TextTemplate;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) struct WidgetVisualResolver;

impl WidgetVisualResolver {
    pub(crate) fn resolve(
        node: &WidgetNode,
        context: &PreviewContext,
        fields: &HashMap<String, Value>,
    ) -> Result<PreviewVisual, String> {
        let mut visual = PreviewVisual::from_node(node);

        visual.class_name = node
            .style
            .class_color_source
            .field()
            .and_then(|key| fields.get(key))
            .and_then(|value| serde_json::from_value(value.clone()).ok());

        visual.simulating = context.simulating;
        visual.animation_value = fields.get(&node.style.animation_path).cloned();
        visual.active = RuleEvaluator::active(node, fields, context.active);
        visual.visible =
            RuleEvaluator::combine(false, &[visual.active, RuleEvaluator::loaded(node, fields)]);
        visual.remaining = fields
            .get("t1.remaining")
            .and_then(Value::as_f64)
            .map(|value| value / 1000.0)
            .or(context.remaining);
        visual.duration = fields
            .get("t1.duration")
            .and_then(Value::as_f64)
            .map(|value| value / 1000.0);
        visual.stacks = fields
            .get("t1.stacks")
            .and_then(Value::as_f64)
            .or(context.stacks);

        if node.kind == WidgetKind::Bar {
            let value =
                match node.style.source {
                    ProgressSource::Fixed => Some(node.style.bar_value),
                    ProgressSource::Field => fields
                        .get(&node.style.path)
                        .and_then(Value::as_f64)
                        .map(|value| {
                            (value - node.style.range_min)
                                / (node.style.range_max - node.style.range_min)
                                * 100.0
                        }),
                    ProgressSource::Trigger => visual
                        .remaining
                        .zip(visual.duration)
                        .filter(|(_, duration)| *duration > 0.0)
                        .map(|(remaining, duration)| remaining / duration * 100.0),
                    ProgressSource::Event => fields
                        .get("event.countdown")
                        .or_else(|| fields.get("event.remaining"))
                        .and_then(Value::as_f64)
                        .zip(fields.get("event.duration").and_then(Value::as_f64))
                        .filter(|(_, duration)| *duration > 0.0)
                        .map(|(remaining, duration)| remaining / duration * 100.0),
                };

            let value = if node.style.source == ProgressSource::Field {
                value
            } else {
                context.progress.or(value)
            };

            visual.known_progress = value.is_some();
            visual.style.bar_value = value.unwrap_or(0.0).clamp(0.0, 100.0);

            if node.style.source == ProgressSource::Field
                && let Some((prefix, _)) = node.style.path.split_once('.')
                && let Some(id) = fields.get(&format!("{prefix}.id")).and_then(Value::as_str)
            {
                visual.progress_identity = Some(format!("{}:{id}", node.style.path));
            }
        }

        ConditionEvaluator::apply(node, &mut visual, context, fields)?;
        PreviewHealing::apply(&mut visual, fields);
        visual.animation_active = match visual.style.animation_when {
            AnimationWhen::Always => true,
            AnimationWhen::EffectActive => visual.active == Some(true),
            AnimationWhen::Remaining => visual
                .remaining
                .is_some_and(|remaining| remaining <= visual.style.animation_threshold),
            AnimationWhen::Elapsed => fields
                .get("event.elapsed")
                .and_then(Value::as_f64)
                .is_some_and(|elapsed| elapsed >= visual.style.animation_threshold * 1000.0),
            AnimationWhen::OnShow => fields
                .get("widget.shownElapsed")
                .and_then(Value::as_f64)
                .is_some_and(|elapsed| elapsed < visual.style.animation_duration),
            AnimationWhen::ValueChanged => fields
                .get(&format!("changed.{}", visual.style.animation_path))
                .and_then(Value::as_f64)
                .is_some_and(|change| match visual.style.animation_change {
                    AnimationChange::Any => change != 0.0,
                    AnimationChange::Increase => change > 0.0,

                    AnimationChange::Decrease => change < 0.0,
                }),
        };
        visual.ticks = visual
            .style
            .ticks
            .iter()
            .filter_map(|value| {
                let fraction = match visual.style.source {
                    ProgressSource::Fixed => Some(value / 100.0),
                    ProgressSource::Field => Some(
                        (value - visual.style.range_min)
                            / (visual.style.range_max - visual.style.range_min),
                    ),
                    _ => visual
                        .duration
                        .filter(|duration| *duration > 0.0)
                        .map(|duration| value / duration),
                }?;

                (0.0..=1.0).contains(&fraction).then_some(fraction)
            })
            .collect();
        visual.appearance.text =
            TextTemplate::render(&visual.appearance.text, fields, context.text());
        visual.style.right_text =
            TextTemplate::render(&visual.style.right_text, fields, context.text());

        Ok(visual)
    }
}
