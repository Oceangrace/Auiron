use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct StorageConfiguration {
    preferences: PathBuf,
    profiles: PathBuf,
    journal: PathBuf,
}

impl StorageConfiguration {
    pub(crate) fn preferences(&self) -> &Path {
        &self.preferences
    }

    pub(crate) fn journal(&self) -> &Path {
        &self.journal
    }

    pub(crate) fn profiles(&self) -> &Path {
        &self.profiles
    }
}
