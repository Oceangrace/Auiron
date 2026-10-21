use crate::core::game::EventRecord;

pub(super) struct CachedEvent {
    signature: String,
    record: EventRecord,
}

impl CachedEvent {
    pub(super) fn new(signature: String, record: EventRecord) -> Self {
        Self { signature, record }
    }

    pub(super) fn signature(&self) -> &str {
        &self.signature
    }

    pub(super) fn record(&self) -> &EventRecord {
        &self.record
    }
}
