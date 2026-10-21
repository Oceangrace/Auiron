use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct BundledLocale {
    id: String,
    label: String,
    ui: PathBuf,
    catalog: Option<PathBuf>,
}

impl BundledLocale {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn ui(&self) -> &Path {
        &self.ui
    }

    pub(crate) fn catalog(&self) -> Option<&Path> {
        self.catalog.as_deref()
    }
}
