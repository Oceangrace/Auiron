use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ResourceConfiguration {
    tray_icon: PathBuf,
}

impl ResourceConfiguration {
    pub(crate) fn tray_icon(&self) -> &Path {
        &self.tray_icon
    }
}
