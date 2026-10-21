use crate::core::game::GameEvent;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RecordedEvent<'a> {
    at_us: u64,
    event: &'a GameEvent,
}

impl<'a> RecordedEvent<'a> {
    pub(super) fn new(at_us: u64, event: &'a GameEvent) -> Self {
        Self { at_us, event }
    }
}
