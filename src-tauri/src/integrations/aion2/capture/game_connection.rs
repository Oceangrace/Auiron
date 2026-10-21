use super::CaptureProcess;
use super::game_endpoint::GameEndpoint;
use crate::integrations::aion2::network::StreamKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct GameConnection {
    process_id: u32,
    created_at: Option<u64>,
    endpoints: Vec<GameEndpoint>,
}

impl GameConnection {
    pub(super) fn process(&self) -> Option<CaptureProcess> {
        self.created_at
            .map(|created| CaptureProcess::new(self.process_id, created))
    }

    pub(super) fn new(
        process_id: u32,
        created_at: Option<u64>,
        mut endpoints: Vec<GameEndpoint>,
    ) -> Self {
        endpoints.sort();
        endpoints.dedup();

        Self {
            process_id,
            created_at,
            endpoints,
        }
    }

    pub(super) fn endpoints(&self) -> &[GameEndpoint] {
        &self.endpoints
    }

    pub(super) fn same_process(&self, other: &Self) -> bool {
        self.process_id == other.process_id && self.created_at == other.created_at
    }

    pub(super) fn accepts(&self, stream: &StreamKey) -> bool {
        self.endpoints
            .iter()
            .any(|endpoint| endpoint.accepts(stream))
    }
}
