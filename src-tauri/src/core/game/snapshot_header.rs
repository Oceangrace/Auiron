use super::SourceStatus;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SnapshotHeader {
    observation_id: String,
    revision: u64,
    at_us: u64,
    status: SourceStatus,
    /// This observation began after the capture lost data; what came before is incomplete.
    interrupted: bool,
}

impl SnapshotHeader {
    pub(crate) fn new(
        observation_id: String,
        revision: u64,
        at_us: u64,
        status: SourceStatus,
    ) -> Self {
        Self {
            observation_id,
            revision,
            at_us,
            status,
            interrupted: false,
        }
    }

    pub(crate) fn with_interrupted(mut self, interrupted: bool) -> Self {
        self.interrupted = interrupted;
        self
    }

    pub(crate) fn interrupted(&self) -> bool {
        self.interrupted
    }

    pub(crate) fn observation_id(&self) -> &str {
        &self.observation_id
    }

    pub(crate) fn at_us(&self) -> u64 {
        self.at_us
    }

    pub(crate) fn status(&self) -> SourceStatus {
        self.status
    }
}
