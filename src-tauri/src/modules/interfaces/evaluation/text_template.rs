use super::PreviewText;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) struct TextTemplate;

impl TextTemplate {
    pub(crate) fn render(
        text: &str,
        fields: &HashMap<String, Value>,
        labels: &PreviewText,
    ) -> String {
        let mut output = String::new();

        let mut restored = false;

        let mut rest = text;

        while let Some(start) = rest.find('{') {
            output.push_str(&rest[..start]);

            let Some(end) = rest[start..].find('}') else {
                output.push_str(&rest[start..]);

                return output;
            };

            let token = &rest[start + 1..start + end];

            let (path, format) = token.split_once(':').unwrap_or((token, ""));

            if path.len() > 128
                || !path
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '-')
            {
                output.push_str(&rest[start..=start + end]);
            } else if let Some(value) = fields.get(path) {
                restored |= fields
                    .get(&format!("restored.{path}"))
                    .and_then(Value::as_bool)
                    == Some(true);
                output.push_str(&Self::format(
                    value,
                    path,
                    format,
                    fields.get(&format!("kind.{path}")),
                    fields.get(&format!("wholeSeconds.{path}")) == Some(&Value::Bool(true)),
                    labels,
                ));
            } else if [
                "t1.",
                "event.",
                "player.",
                "zone.",
                "worldBoss.",
                "group.",
                "entity.",
                "target.",
                "targetTarget.",
                "combat.",
                "encounter.",
                "row.",
                "formula.",
                "party",
            ]
            .iter()
            .any(|prefix| path.starts_with(prefix))
            {
                output.push('?');
            } else {
                output.push_str(&rest[start..=start + end]);
            }

            rest = &rest[start + end + 1..];
        }

        output.push_str(rest);

        if restored {
            return format!("≈ {output}");
        }

        output
    }

    fn format(
        value: &Value,
        path: &str,
        format: &str,
        kind: Option<&Value>,
        whole_seconds: bool,
        labels: &PreviewText,
    ) -> String {
        if let Some(value) = value.as_str() {
            return value.to_owned();
        }

        if let Some(value) = value.as_bool() {
            return labels.boolean(value).to_owned();
        }

        let Some(value) = value.as_f64() else {
            return "?".into();
        };

        if format == "k" {
            return if value.abs() >= 1_000_000.0 {
                format!("{:.1}M", value / 1_000_000.0)
            } else if value.abs() >= 1000.0 {
                format!("{:.1}k", value / 1000.0)
            } else {
                format!("{value:.0}")
            };
        }

        if !format.is_empty() {
            let decimals = format
                .split_once('.')
                .map_or(0, |(_, digits)| digits.len())
                .min(6);

            return format!("{value:.decimals$}");
        }

        if kind.is_some_and(|value| value == "duration")
            || [
                "remaining",
                "duration",
                "elapsed",
                "countdown",
                "battleRemaining",
            ]
            .iter()
            .any(|part| path.ends_with(part))
        {
            if whole_seconds {
                return Self::whole_duration(value / 1000.0);
            }

            return Self::duration(value / 1000.0);
        }

        format!("{value:.0}")
    }

    fn whole_duration(seconds: f64) -> String {
        let seconds = seconds.max(0.0).ceil();

        if seconds < 60.0 {
            return format!("{seconds:.0}");
        }

        Self::duration(seconds)
    }

    pub(crate) fn duration(seconds: f64) -> String {
        let seconds = seconds.max(0.0);

        if seconds < 10.0 {
            format!("{seconds:.1}")
        } else if seconds < 60.0 {
            format!("{seconds:.0}", seconds = seconds.floor())
        } else {
            format!(
                "{}:{:02}",
                (seconds / 60.0).floor() as u64,
                seconds.floor() as u64 % 60
            )
        }
    }
}
