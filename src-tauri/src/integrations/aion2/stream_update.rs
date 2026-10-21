use super::StreamReset;
use crate::core::game::GameEvent;

pub(crate) struct StreamUpdate {
    reset: Option<StreamReset>,
    events: Vec<GameEvent>,
}

impl StreamUpdate {
    pub(crate) fn new(reset: Option<StreamReset>, events: Vec<GameEvent>) -> Self {
        Self { reset, events }
    }

    pub(crate) fn reset(&self) -> Option<&StreamReset> {
        self.reset.as_ref()
    }

    pub(crate) fn events(&self) -> &[GameEvent] {
        &self.events
    }
}
