use crate::core::game::GameObservationSource;
use crate::core::game::SourceFrame;
use std::sync::{Arc, Mutex};

pub(super) struct SourceReader {
    latest: Arc<Mutex<Arc<SourceFrame>>>,
}

impl SourceReader {
    pub(super) fn new(latest: Arc<Mutex<Arc<SourceFrame>>>) -> Self {
        Self { latest }
    }
}

impl GameObservationSource for SourceReader {
    fn frame(&self) -> Result<Arc<SourceFrame>, String> {
        self.latest
            .lock()
            .map(|frame| Arc::clone(&frame))
            .map_err(|_| "Game source state is unavailable".into())
    }
}
