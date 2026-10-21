use crate::core::localization::LanguagePack;
use crate::core::localization::LocaleOption;
use icu_collator::Collator;
use icu_collator::CollatorBorrowed;
use serde_json::{Value, json};
use std::collections::HashMap;

pub(crate) struct LocaleRegistry {
    packs: HashMap<String, LanguagePack>,
    collators: HashMap<String, CollatorBorrowed<'static>>,
    fallback: String,
    default: String,
    warnings: Vec<String>,
    zone_values: Vec<String>,
}

impl LocaleRegistry {
    pub(crate) fn new(
        packs: Vec<LanguagePack>,
        fallback: String,
        default: String,
        warnings: Vec<String>,
        zone_values: Vec<String>,
    ) -> Result<Self, String> {
        let mut registry = Self {
            packs: HashMap::new(),
            collators: HashMap::new(),
            fallback,
            default,
            warnings,
            zone_values,
        };

        for pack in packs {
            pack.validate()?;
            let locale = pack
                .id()
                .parse::<icu_locale::Locale>()
                .map_err(|error| error.to_string())?;

            let collator = Collator::try_new(locale.into(), Default::default())
                .map_err(|error| error.to_string())?;

            registry.collators.insert(pack.id().to_owned(), collator);

            if registry.packs.insert(pack.id().to_owned(), pack).is_some() {
                return Err("Duplicate locale ID".into());
            }
        }

        if !registry.packs.contains_key(&registry.default)
            || !registry.packs.contains_key(&registry.fallback)
        {
            return Err("Default or fallback locale is missing".into());
        }

        Ok(registry)
    }

    pub(crate) fn validate_catalogs(&self, version: &str) -> Result<(), String> {
        if self
            .packs
            .values()
            .any(|pack| !pack.catalog().is_null() && pack.catalog()["catalogVersion"] != version)
        {
            return Err("Language catalog version mismatch".into());
        }

        Ok(())
    }

    pub(crate) fn default_locale(&self) -> &str {
        &self.default
    }

    pub(crate) fn fallback_locale(&self) -> &str {
        &self.fallback
    }

    pub(crate) fn languages(&self) -> impl Iterator<Item = &str> {
        self.packs.keys().map(String::as_str)
    }

    pub(crate) fn pack(&self, locale: &str) -> &LanguagePack {
        &self.packs[self.effective(locale)]
    }

    pub(crate) fn effective<'a>(&'a self, locale: &'a str) -> &'a str {
        if locale.is_empty() {
            return &self.default;
        }

        if self.packs.contains_key(locale) {
            return locale;
        }

        &self.fallback
    }

    pub(crate) fn text(&self, locale: &str, key: &str) -> String {
        self.message(locale, &["native", key])
    }

    pub(crate) fn message(&self, locale: &str, path: &[&str]) -> String {
        let local = path
            .iter()
            .fold(self.pack(locale).messages(), |value, key| &value[*key]);

        let fallback = path
            .iter()
            .fold(self.packs[&self.fallback].messages(), |value, key| {
                &value[*key]
            });

        local
            .as_str()
            .or_else(|| fallback.as_str())
            .map(str::to_owned)
            .unwrap_or_else(|| path.join("."))
    }

    pub(crate) fn compare(&self, locale: &str, left: &str, right: &str) -> std::cmp::Ordering {
        self.collators[self.effective(locale)].compare(left, right)
    }

    pub(crate) fn snapshot(&self) -> Value {
        let mut languages: Vec<LocaleOption> = self
            .packs
            .values()
            .map(|pack| LocaleOption {
                value: pack.id().to_owned(),
                label: pack.label().to_owned(),
            })
            .collect();

        languages.sort_by(|a, b| a.value.cmp(&b.value));
        let messages: HashMap<_, _> = self
            .packs
            .values()
            .map(|pack| (pack.id().to_owned(), self.messages(pack.id())))
            .collect();

        let zones: HashMap<_, _> = self
            .packs
            .values()
            .map(|pack| (pack.id().to_owned(), self.zones(pack.id())))
            .collect();

        json!({"languages":languages,"messages":messages,"zoneValues":self.zone_values,"zoneNames":zones,"fallbackLocale":self.fallback,"defaultLocale":self.default,"warnings":self.warnings})
    }

    fn messages(&self, locale: &str) -> Value {
        let mut value = self.packs[&self.fallback].messages().clone();

        Self::merge(&mut value, self.pack(locale).messages());

        value
    }

    fn zones(&self, locale: &str) -> Value {
        let mut zones = self.packs[&self.fallback].zones().clone();

        if self.pack(locale).zones().is_object() {
            Self::merge(&mut zones, self.pack(locale).zones());
        }

        zones
    }

    fn merge(target: &mut Value, source: &Value) {
        if let (Some(target), Some(source)) = (target.as_object_mut(), source.as_object()) {
            for (key, value) in source {
                Self::merge(target.entry(key).or_insert(Value::Null), value);
            }

            return;
        }

        *target = source.clone();
    }
}
