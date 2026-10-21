use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct LanguagePackageConfiguration {
    manifest: PathBuf,
    messages: PathBuf,
    catalog: PathBuf,
}

impl LanguagePackageConfiguration {
    pub(crate) fn manifest(&self) -> &Path {
        &self.manifest
    }

    pub(crate) fn messages(&self) -> &Path {
        &self.messages
    }

    pub(crate) fn catalog(&self) -> &Path {
        &self.catalog
    }
}
