use serde::{Deserialize, Serialize};

/// Creation time prevents a reused Windows PID from identifying a previous game session.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct CaptureProcess {
    id: u32,
    created_at: u64,
}

impl CaptureProcess {
    pub(crate) fn new(id: u32, created_at: u64) -> Self {
        Self { id, created_at }
    }
}
