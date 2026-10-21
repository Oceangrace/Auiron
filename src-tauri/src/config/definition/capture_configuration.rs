use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct CaptureConfiguration {
    process_name: String,
    library: PathBuf,
    #[schemars(range(min = 100, max = 60000))]
    refresh_ms: u64,
    #[schemars(range(min = 1, max = 1000))]
    poll_ms: u64,
    #[schemars(range(min = 50, max = 1000))]
    publish_ms: u64,
    #[schemars(range(min = 1000, max = 60000))]
    stale_ms: u64,
}

impl CaptureConfiguration {
    pub(crate) fn process_name(&self) -> &str {
        &self.process_name
    }

    pub(crate) fn library(&self) -> &Path {
        &self.library
    }

    pub(crate) fn refresh_ms(&self) -> u64 {
        self.refresh_ms
    }

    pub(crate) fn poll_ms(&self) -> u64 {
        self.poll_ms
    }

    pub(crate) fn publish_ms(&self) -> u64 {
        self.publish_ms
    }

    pub(crate) fn stale_ms(&self) -> u64 {
        self.stale_ms
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        super::ConfigurationPaths::filename(Path::new(&self.process_name))?;
        super::ConfigurationPaths::relative(&self.library)?;

        if !self.process_name.to_ascii_lowercase().ends_with(".exe")
            || !(100..=60000).contains(&self.refresh_ms)
            || !(1..=1000).contains(&self.poll_ms)
            || !(50..=1000).contains(&self.publish_ms)
            || !(1000..=60000).contains(&self.stale_ms)
        {
            return Err("Invalid game capture configuration".into());
        }

        Ok(())
    }
}
