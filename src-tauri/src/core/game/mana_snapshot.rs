use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ManaSnapshot {
    current: Option<u32>,
    maximum: Option<u32>,
    #[serde(default)]
    restored: bool,
}

impl ManaSnapshot {
    pub(crate) fn new(current: Option<u32>, maximum: Option<u32>) -> Self {
        Self {
            current,
            maximum,
            restored: false,
        }
    }

    pub(crate) fn with_restored(mut self, restored: bool) -> Self {
        self.restored = restored;
        self
    }

    pub(crate) fn restored(&self) -> bool {
        self.restored
    }

    pub(crate) fn current(&self) -> Option<u32> {
        self.current
    }

    pub(crate) fn maximum(&self) -> Option<u32> {
        self.maximum
    }
}
