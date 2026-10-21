use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub(crate) struct InterfaceLocalization {
    pub(in crate::modules::interfaces) default_locale: String,
    pub(in crate::modules::interfaces) translations: BTreeMap<String, BTreeMap<String, String>>,
}

impl InterfaceLocalization {
    pub(in crate::modules::interfaces) fn validate(&self) -> Result<(), String> {
        if (!self.default_locale.is_empty() && !Self::locale(&self.default_locale))
            || self.translations.len() > 32
        {
            return Err("Invalid HUD source language or too many translations".into());
        }

        let mut total = 0;

        for (locale, texts) in &self.translations {
            if !Self::locale(locale) || locale == &self.default_locale || texts.len() > 8192 {
                return Err("Invalid HUD translation language or too many texts".into());
            }

            for (source, target) in texts {
                total += source.len() + target.len();

                if source.trim().is_empty()
                    || source.chars().count() > 2048
                    || target.chars().count() > 2048
                    || total > 2_000_000
                {
                    return Err("HUD translations exceed the text limit".into());
                }

                if !target.is_empty() && Self::tokens(source) != Self::tokens(target) {
                    return Err(format!(
                        "HUD translation must preserve template fields ({locale}): {source}"
                    ));
                }
            }
        }

        Ok(())
    }

    pub(in crate::modules::interfaces) fn dictionary(
        &self,
        locale: &str,
    ) -> Option<BTreeMap<&str, &str>> {
        let mut code = locale.to_ascii_lowercase().replace('_', "-");
        let mut texts = BTreeMap::new();

        loop {
            if let Some(language) = self.translations.get(&code) {
                for (source, target) in language {
                    if !target.is_empty() {
                        texts.entry(source.as_str()).or_insert(target.as_str());
                    }
                }
            }

            let Some((parent, _)) = code.rsplit_once('-') else {
                break;
            };

            code = parent.to_owned();
        }

        if texts.is_empty() { None } else { Some(texts) }
    }

    fn locale(value: &str) -> bool {
        let Some(language) = value.split('-').next() else {
            return false;
        };

        (2..=8).contains(&language.len())
            && language.bytes().all(|byte| byte.is_ascii_lowercase())
            && value.len() <= 35
            && value.split('-').all(|part| {
                !part.is_empty()
                    && part.len() <= 8
                    && part
                        .bytes()
                        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            })
    }

    pub(in crate::modules::interfaces) fn tokens(value: &str) -> Vec<&str> {
        let mut rest = value;
        let mut tokens = Vec::new();

        while let Some(start) = rest.find('{') {
            let Some(end) = rest[start..].find('}') else {
                break;
            };

            tokens.push(&rest[start..=start + end]);
            rest = &rest[start + end + 1..];
        }

        tokens.sort_unstable();

        tokens
    }
}
