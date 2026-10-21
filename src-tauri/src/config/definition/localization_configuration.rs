use super::BundledLocale;
use super::LanguagePackageConfiguration;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LocalizationConfiguration {
    default_locale: String,
    fallback_locale: String,
    directory: PathBuf,
    bundled: Vec<BundledLocale>,
    package: LanguagePackageConfiguration,
}

impl LocalizationConfiguration {
    pub(crate) fn default_locale(&self) -> &str {
        &self.default_locale
    }

    pub(crate) fn fallback_locale(&self) -> &str {
        &self.fallback_locale
    }

    pub(crate) fn directory(&self) -> &Path {
        &self.directory
    }

    pub(crate) fn bundled(&self) -> &[BundledLocale] {
        &self.bundled
    }

    pub(crate) fn package(&self) -> &LanguagePackageConfiguration {
        &self.package
    }
}
