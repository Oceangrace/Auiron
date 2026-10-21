use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::evaluation::PreviewContext;
use crate::modules::interfaces::evaluation::PreviewVisual;
use crate::modules::interfaces::evaluation::VisualChanges;
use crate::modules::interfaces::rules::ConditionField;
use base64::Engine;
use std::collections::HashMap;
use std::collections::HashSet;

pub(crate) struct WidgetVisualValidator;

impl WidgetVisualValidator {
    pub(crate) fn validate(node: &WidgetNode) -> Result<(), String> {
        if let Some(image) = &node.appearance.custom_image {
            let (header, data) = image.split_once(',').ok_or("Invalid image")?;

            if image.len() > 200_000
                || ![
                    "data:image/png;base64",
                    "data:image/jpeg;base64",
                    "data:image/webp;base64",
                ]
                .contains(&header)
            {
                return Err("Image must be PNG, JPEG or WebP up to 150 KB".into());
            }

            let bytes = base64::engine::general_purpose::STANDARD
                .decode(data)
                .map_err(|_| "Invalid image")?;

            let signature = bytes.starts_with(b"\x89PNG\r\n\x1a\n")
                || bytes.starts_with(b"\xff\xd8\xff")
                || (bytes.starts_with(b"RIFF") && bytes.get(8..12) == Some(b"WEBP"));

            if !signature {
                return Err("Invalid image format".into());
            }
        }

        let style = &node.style;

        Self::number(style.border_width, 0.0, 20.0)?;
        Self::number(style.radius, 0.0, 100.0)?;
        Self::number(style.period_ms, 100.0, 10000.0)?;
        Self::number(style.bar_value, 0.0, 100.0)?;
        VisualChanges::color(&style.fill_color)?;
        VisualChanges::color(&style.glow_color)?;
        VisualChanges::color(&style.healing_color)?;
        VisualChanges::color(&style.overheal_color)?;

        if let Some(color) = &style.bar_loss_color {
            VisualChanges::color(color)?;
        }

        Self::number(style.overheal_texture_size, 2.0, 32.0)?;
        let group = &node.group;

        Self::number(group.header_height, 20.0, 100.0)?;
        Self::number(group.scale, 25.0, 400.0)?;
        Self::number(group.padding, 0.0, 100.0)?;
        Self::number(group.gap, 0.0, 200.0)?;
        Self::number(group.columns, 1.0, 100.0)?;

        if group.columns.fract() != 0.0 || group.header_text.chars().count() > 256 {
            return Err("Invalid group settings".into());
        }

        VisualChanges::color(&group.header_color)?;
        VisualChanges::color(&group.header_background)?;

        if node.conditions.len() > 32 {
            return Err("Too many conditions".into());
        }

        let mut ids = HashSet::new();

        let mut visual = PreviewVisual::from_node(node);

        for condition in &node.conditions {
            if condition.id.is_empty()
                || condition.id.len() > 100
                || !ids.insert(&condition.id)
                || condition.checks.is_empty()
                || condition.checks.len() > 8
                || condition.changes.len() > 32
            {
                return Err("Invalid condition structure".into());
            }

            if condition
                .changes
                .iter()
                .any(|change| change.speech_text().chars().count() > 160)
            {
                return Err("Condition phrase exceeds 160 characters".into());
            }

            for check in &condition.checks {
                let max = match check.field {
                    ConditionField::Active => 1.0,
                    ConditionField::Remaining => 3600.0,
                    ConditionField::Stacks => 999.0,
                    ConditionField::Progress => 100.0,
                    ConditionField::Custom => 1e9,
                };

                if check.field != ConditionField::Custom {
                    Self::number(check.value, 0.0, max)?;

                    continue;
                }

                if check
                    .path
                    .as_ref()
                    .is_none_or(|path| path.is_empty() || path.len() > 128)
                    || check
                        .text_value
                        .as_ref()
                        .is_none_or(|value| value.len() > 128)
                {
                    return Err("Invalid condition field".into());
                }
            }

            for change in &condition.changes {
                VisualChanges::apply(&mut visual, change)?;
            }
        }

        Ok(())
    }

    pub(crate) fn context(context: &PreviewContext) -> Result<(), String> {
        for (value, max) in [
            (context.remaining, 3600.0),
            (context.stacks, 999.0),
            (context.progress, 100.0),
        ] {
            if let Some(value) = value {
                Self::number(value, 0.0, max)?;
            }
        }

        if context.fields.len() > 256
            || context.rows.len() > 128
            || context.instances.len() > 128
            || context.widgets.len() > 1024
            || context.widgets.values().map(HashMap::len).sum::<usize>() > 16384
        {
            return Err("Too much test data".into());
        }

        let validate = |fields: &HashMap<String, serde_json::Value>| -> Result<(), String> {
            if fields.len() > 256
                || fields.iter().any(|(key, value)| {
                    key.is_empty()
                        || key.len() > 128
                        || !(value.is_null()
                            || value.is_boolean()
                            || value.as_f64().is_some_and(f64::is_finite)
                            || value.as_str().is_some_and(|text| text.len() <= 2048))
                })
            {
                return Err("Invalid check field".into());
            }

            Ok(())
        };

        validate(&context.fields)?;

        for fields in context.widgets.values() {
            validate(fields)?;
        }

        for row in &context.rows {
            validate(row)?;
        }

        for instances in context.instances.values() {
            if instances.len() > 128 {
                return Err("Too many instances".into());
            }

            for instance in instances {
                validate(instance)?;
            }
        }

        Ok(())
    }

    fn number(value: f64, min: f64, max: f64) -> Result<(), String> {
        if !value.is_finite() || !(min..=max).contains(&value) {
            return Err("Appearance setting is out of range".into());
        }

        Ok(())
    }
}
