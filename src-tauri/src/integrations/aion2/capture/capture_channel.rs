use super::CapturedPacket;
use super::game_connection::GameConnection;
use super::game_endpoint::GameEndpoint;
use super::npcap_api::NpcapApi;
use super::npcap_handle::NpcapHandle;
use std::ffi::{CStr, CString};
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Keep a server filter across local port changes; LiveTransport verifies socket ownership.
pub(super) struct CaptureChannel {
    adapter: Vec<u8>,
    endpoint: GameEndpoint,
    handle: NpcapHandle,
    retired_at: Option<Instant>,
}

impl CaptureChannel {
    // Keep the previous server through a character selection/loading interval.
    const RETIRE_DELAY: Duration = Duration::from_secs(60);

    pub(super) fn open(
        api: Rc<NpcapApi>,
        adapter: &CStr,
        endpoint: GameEndpoint,
    ) -> Result<Self, String> {
        let filter = CString::new(endpoint.filter()).map_err(|error| error.to_string())?;

        let handle = NpcapHandle::open(api, adapter, &filter)?;

        Ok(Self {
            adapter: adapter.to_bytes().to_vec(),
            endpoint,
            handle,
            retired_at: None,
        })
    }

    pub(super) fn matches(&self, adapter: &CStr, endpoint: &GameEndpoint) -> bool {
        self.adapter == adapter.to_bytes() && self.endpoint.same_server(endpoint)
    }

    pub(super) fn refresh(&mut self, connection: &GameConnection) {
        if connection
            .endpoints()
            .iter()
            .any(|endpoint| self.endpoint.same_server(endpoint))
        {
            self.retired_at = None;

            return;
        }

        self.retired_at.get_or_insert_with(Instant::now);
    }

    pub(super) fn expired(&self) -> bool {
        self.retired_at
            .is_some_and(|retired| retired.elapsed() >= Self::RETIRE_DELAY)
    }

    pub(super) fn next(&mut self) -> Result<Option<CapturedPacket>, String> {
        self.handle.next()
    }

    pub(super) fn lost_packets(&mut self) -> Result<bool, String> {
        self.handle.lost_packets()
    }
}
