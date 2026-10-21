use super::MapId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InstanceTimer {
    map_id: MapId,
    kind: u8,
    started_at_ms: u64,
    ends_at_ms: Option<u64>,
}

impl InstanceTimer {
    pub(crate) fn new(
        map_id: MapId,
        kind: u8,
        started_at_ms: u64,
        ends_at_ms: Option<u64>,
    ) -> Self {
        Self {
            map_id,
            kind,
            started_at_ms,
            ends_at_ms,
        }
    }

    pub(crate) fn map_id(&self) -> MapId {
        self.map_id
    }

    pub(crate) fn kind(&self) -> u8 {
        self.kind
    }

    pub(crate) fn started_at_ms(&self) -> u64 {
        self.started_at_ms
    }

    pub(crate) fn ends_at_ms(&self) -> Option<u64> {
        self.ends_at_ms
    }
}
