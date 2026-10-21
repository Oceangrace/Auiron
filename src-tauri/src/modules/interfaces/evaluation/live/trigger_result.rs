use super::trigger_match::TriggerMatch;
use super::trigger_state::TriggerState;

pub(super) struct TriggerResult {
    state: TriggerState,
    matches: Vec<TriggerMatch>,
}

impl TriggerResult {
    pub(super) fn new(state: TriggerState, matches: Vec<TriggerMatch>) -> Self {
        Self { state, matches }
    }

    pub(super) fn state(&self) -> TriggerState {
        self.state
    }

    pub(super) fn matches(&self) -> &[TriggerMatch] {
        &self.matches
    }
}
