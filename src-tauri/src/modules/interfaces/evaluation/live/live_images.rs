use super::selection_resolver::SelectionResolver;
use crate::modules::interfaces::appearance::ImageSource;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewContext;
use crate::modules::interfaces::evaluation::PreviewVisual;
use serde_json::Value;
use std::collections::HashMap;

pub(super) struct LiveImages;

impl LiveImages {
    pub(super) fn apply(
        node: &WidgetNode,
        context: &PreviewContext,
        resolver: &SelectionResolver<'_>,
        visuals: &mut HashMap<String, PreviewVisual>,
    ) {
        if node.appearance.custom_image.is_some() {
            return;
        }

        let Some(visual) = visuals.get_mut(&node.id) else {
            return;
        };

        if node.style.image_source == ImageSource::Skill {
            Self::skill(node, visual, context, resolver);

            return;
        }

        if node.style.image_source != ImageSource::Automatic {
            return;
        }

        let prefix = node
            .rules
            .as_ref()
            .and_then(|rules| rules.data_rule_id.as_ref())
            .map_or_else(|| "t1.".to_owned(), |id| format!("rule.{id}."));

        Self::resolve(visual, &context.fields, resolver, &prefix);

        if let Some(id) = Self::id(&context.fields, "skill.displayId") {
            visual.appearance.icon_path = resolver.icon(None, Some(id)).map(str::to_owned);
        }

        if let Some(instances) = context.instances.get(&node.id) {
            for (visual, instance) in visual.instances.iter_mut().zip(instances) {
                Self::resolve(&mut visual.visual, instance, resolver, &prefix);
            }
        }
    }

    fn skill(
        node: &WidgetNode,
        visual: &mut PreviewVisual,
        context: &PreviewContext,
        resolver: &SelectionResolver<'_>,
    ) {
        let Some(selection) = node.image_selection.as_ref() else {
            return;
        };

        // A selected skill follows its chain; an explicitly fixed image without a selection does not.
        if node
            .trigger
            .as_ref()
            .is_none_or(|trigger| trigger.selection.entry_key != selection.entry_key)
        {
            return;
        }

        if let Some(id) = Self::id(&context.fields, "skill.displayId") {
            visual.appearance.icon_path = resolver.icon(None, Some(id)).map(str::to_owned);
        }
    }

    fn resolve(
        visual: &mut PreviewVisual,
        fields: &HashMap<String, Value>,
        resolver: &SelectionResolver<'_>,
        prefix: &str,
    ) {
        if let Some(icon) = fields
            .get(&format!("{prefix}iconPath"))
            .and_then(Value::as_str)
        {
            visual.appearance.icon_path = Some(icon.to_owned());
            return;
        }

        let effect = Self::id(fields, &format!("{prefix}effectId"));

        let skill = Self::id(fields, &format!("{prefix}skillId"));

        if effect.is_some() || skill.is_some() {
            visual.appearance.icon_path = resolver.icon(effect, skill).map(str::to_owned);
        }
    }

    fn id(fields: &HashMap<String, Value>, key: &str) -> Option<u32> {
        fields
            .get(key)
            .and_then(Value::as_u64)
            .and_then(|id| u32::try_from(id).ok())
    }
}
