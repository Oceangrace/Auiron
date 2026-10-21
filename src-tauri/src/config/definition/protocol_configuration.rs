use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ProtocolConfiguration {
    table: PathBuf,
    #[schemars(range(min = 1000, max = 120000))]
    combat_idle_ms: u64,
    capture: super::CaptureConfiguration,
}

impl ProtocolConfiguration {
    pub(crate) fn validate(&self) -> Result<(), String> {
        self.capture.validate()?;

        if !(1000..=120000).contains(&self.combat_idle_ms) {
            return Err("Invalid combat inactivity timeout".into());
        }

        Ok(())
    }

    pub(crate) fn combat_idle_ms(&self) -> u64 {
        self.combat_idle_ms
    }

    pub(crate) fn capture(&self) -> &super::CaptureConfiguration {
        &self.capture
    }

    pub(crate) fn table(&self) -> &Path {
        &self.table
    }
}
