use super::GameEvent;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EventRecord {
    sequence: u64,
    at_us: u64,
    event: GameEvent,
}

impl EventRecord {
    pub(crate) fn new(sequence: u64, at_us: u64, event: GameEvent) -> Self {
        Self {
            sequence,
            at_us,
            event,
        }
    }

    pub(crate) fn sequence(&self) -> u64 {
        self.sequence
    }

    pub(crate) fn at_us(&self) -> u64 {
        self.at_us
    }

    pub(crate) fn event(&self) -> &GameEvent {
        &self.event
    }
}
