use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LanguagePack {
    id: String,
    label: String,
    messages: Value,
    #[serde(default)]
    catalog: Value,
    #[serde(default)]
    zones: Value,
}

impl LanguagePack {
    pub(crate) fn new(
        id: String,
        label: String,
        messages: Value,
        catalog: Value,
        zones: Value,
    ) -> Self {
        Self {
            id,
            label,
            messages,
            catalog,
            zones,
        }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn messages(&self) -> &Value {
        &self.messages
    }

    pub(crate) fn catalog(&self) -> &Value {
        &self.catalog
    }

    pub(crate) fn zones(&self) -> &Value {
        &self.zones
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        self.id
            .parse::<icu_locale::Locale>()
            .map_err(|_| "Invalid locale ID")?;

        if self.id.len() > 63 || self.label.trim().is_empty() || self.label.chars().count() > 80 {
            return Err("Invalid language manifest".into());
        }

        if !self.messages.is_object() {
            return Err("Language UI must be an object".into());
        }

        Self::validate_messages(&self.messages, 0)?;

        if self
            .catalog
            .get("entries")
            .is_some_and(|entries| !entries.is_object())
        {
            return Err("Invalid catalog translation entries".into());
        }

        if let Some(entries) = self.catalog["entries"].as_object() {
            for item in entries.values() {
                Self::text_fields(item, &["name", "context", "forms"])?;

                if item.get("forms").is_some_and(|forms| !forms.is_object()) {
                    return Err("Invalid catalog translation forms".into());
                }

                if let Some(forms) = item["forms"].as_object() {
                    for form in forms.values() {
                        Self::text_fields(form, &["name", "description"])?;
                    }
                }
            }
        }

        if !self.zones.is_null() {
            Self::validate_messages(&self.zones, 0)?;
        }

        Ok(())
    }

    fn validate_messages(value: &Value, depth: usize) -> Result<(), String> {
        if depth > 16 {
            return Err("Translation nesting is too deep".into());
        }

        match value {
            Value::String(text) if text.len() <= 16384 => Ok(()),
            Value::Object(entries) if entries.len() <= 12000 => {
                for (key, item) in entries {
                    if ["__proto__", "prototype", "constructor"].contains(&key.as_str()) {
                        return Err("Invalid translation key".into());
                    }

                    Self::validate_messages(item, depth + 1)?;
                }

                Ok(())
            }
            _ => Err("Translations must contain objects and strings".into()),
        }
    }

    fn text_fields(value: &Value, allowed: &[&str]) -> Result<(), String> {
        let entries = value.as_object().ok_or("Invalid catalog translation")?;

        for (key, text) in entries {
            if !allowed.contains(&key.as_str()) {
                return Err(format!("Catalog translation cannot change {key}"));
            }

            if key != "forms" && !text.is_null() && !text.is_string() {
                return Err("Invalid catalog text".into());
            }

            if text.as_str().is_some_and(|value| value.len() > 16384) {
                return Err("Catalog text is too long".into());
            }
        }

        Ok(())
    }
}
