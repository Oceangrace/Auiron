use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HealthSnapshot {
    current: Option<u32>,
    maximum: Option<u32>,
    #[serde(default)]
    restored: bool,
    #[serde(default)]
    observed_at_us: u64,
}

impl HealthSnapshot {
    pub(crate) fn new(current: Option<u32>, maximum: Option<u32>) -> Self {
        Self {
            current,
            maximum,
            restored: false,
            observed_at_us: 0,
        }
    }

    pub(crate) fn with_restored(mut self, restored: bool) -> Self {
        self.restored = restored;
        self
    }

    pub(crate) fn restored(&self) -> bool {
        self.restored
    }

    pub(crate) fn with_observed_at(mut self, at_us: u64) -> Self {
        self.observed_at_us = at_us;
        self
    }

    pub(crate) fn observed_at_us(&self) -> u64 {
        self.observed_at_us
    }

    pub(crate) fn current(&self) -> Option<u32> {
        self.current
    }

    pub(crate) fn maximum(&self) -> Option<u32> {
        self.maximum
    }
}
