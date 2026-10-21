pub(super) struct ServerClock {
    offset_us: Option<i128>,
}

impl ServerClock {
    pub(super) fn new() -> Self {
        Self { offset_us: None }
    }

    pub(super) fn sync(&mut self, server_ms: u64, at_us: u64) {
        self.offset_us = Some(i128::from(server_ms) * 1000 - i128::from(at_us));
    }

    pub(super) fn to_local(&self, server_ms: u64) -> Option<u64> {
        let offset = self.offset_us?;

        u64::try_from(i128::from(server_ms) * 1000 - offset).ok()
    }
}
