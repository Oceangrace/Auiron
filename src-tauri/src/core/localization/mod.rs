//! Языковые пакеты, выбор переводов и правила сортировки.

mod language_manifest;
mod language_pack;
mod locale_option;
mod locale_registry;
mod text_matcher;

pub(crate) use self::language_manifest::LanguageManifest;
pub(crate) use self::language_pack::LanguagePack;
pub(crate) use self::locale_option::LocaleOption;
pub(crate) use self::locale_registry::LocaleRegistry;
pub(crate) use self::text_matcher::TextMatcher;
