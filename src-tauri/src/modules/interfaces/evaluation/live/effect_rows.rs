use super::effect_matcher::EffectMatcher;
use super::live_inputs::LiveInputs;
use super::trigger_match::TriggerMatch;
use crate::core::game::SourceStatus;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewInstance;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::evaluation::RuleEvaluator;
use crate::modules::interfaces::layout::GroupSort;
use crate::modules::interfaces::rules::EffectOrigin;
use crate::modules::interfaces::rules::EffectPresence;
use crate::modules::interfaces::rules::EffectTrigger;

pub(super) struct EffectRows;

impl EffectRows {
    pub(super) fn apply(node: &WidgetNode, inputs: &LiveInputs<'_>, visual: &mut PreviewVisual) {
        let Some(binding) = &node.binding else {
            return;
        };

        let Some(settings) = &binding.effects else {
            return;
        };

        visual.instances.clear();

        if !matches!(
            inputs.game().header().status(),
            SourceStatus::Live | SourceStatus::Replay
        ) {
            visual.active = None;
            visual.visible = None;
            return;
        }

        let trigger = EffectTrigger {
            unit: settings.unit,
            member: String::new(),
            filter: settings.filter,
            effects: Vec::new(),
            presence: EffectPresence::Present,
            origin: EffectOrigin::Any,
            min_stacks: 1.0,
            remaining_below_seconds: 0.0,
            clones: false,
            clone_direction: Default::default(),
        };

        let result = EffectMatcher::evaluate(&trigger, None, inputs.game());

        visual.active = RuleEvaluator::combine(false, &[visual.active, result.state().active()]);
        visual.visible = RuleEvaluator::combine(false, &[visual.visible, result.state().active()]);

        let mut entries = result
            .matches()
            .iter()
            .filter_map(|item| Self::instance(node, inputs, item))
            .collect::<Vec<_>>();

        Self::sort(node, &mut entries);

        let limit = if node.group.limit > 0.0 {
            binding.limit.min(node.group.limit as u32)
        } else {
            binding.limit
        };

        entries.truncate(limit as usize);

        for entry in &mut entries {
            entry.visual.class_name = visual.class_name;
        }

        visual.instances = entries;
    }

    fn instance(
        node: &WidgetNode,
        inputs: &LiveInputs<'_>,
        item: &TriggerMatch,
    ) -> Option<PreviewInstance> {
        let TriggerMatch::Effect {
            entity_id,
            instances,
        } = item
        else {
            return None;
        };

        let snapshot = instances.first()?;
        let effect = snapshot.effect();
        let display = snapshot.display();

        if display.is_some_and(|display| !display.icon_visible()) {
            return None;
        }

        let fields = item.fields(inputs.game());

        let mut visual = PreviewVisual::from_node(node);

        visual.simulating = true;
        visual.active = Some(true);
        visual.visible = Some(true);

        if display.is_none_or(|display| display.remaining_time_visible()) {
            visual.remaining = fields
                .get("remaining")
                .and_then(serde_json::Value::as_f64)
                .map(|v| v / 1000.0);
            visual.duration = fields
                .get("duration")
                .and_then(serde_json::Value::as_f64)
                .map(|v| v / 1000.0);
        }

        visual.stacks = Some(instances.len() as f64);
        visual.appearance.text.clear();
        visual.appearance.custom_image = None;
        visual.appearance.icon_path = display
            .and_then(|display| display.icon())
            .or_else(|| {
                inputs.resolver().icon(
                    Some(effect.effect_id().get()),
                    effect.skill_id().map(|id| id.get()),
                )
            })
            .map(str::to_owned);

        Some(PreviewInstance {
            key: format!(
                "{}:{}:{:?}",
                entity_id.get(),
                effect.effect_id().get(),
                effect.source_id()
            ),
            visual: Box::new(visual),
        })
    }

    fn sort(node: &WidgetNode, entries: &mut [PreviewInstance]) {
        if node.group.sort == GroupSort::Manual {
            return;
        }

        entries.sort_by(
            |left, right| match (left.visual.remaining, right.visual.remaining) {
                (Some(left), Some(right)) if node.group.sort == GroupSort::RemainingDesc => {
                    right.total_cmp(&left)
                }
                (Some(left), Some(right)) => left.total_cmp(&right),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            },
        );
    }
}
