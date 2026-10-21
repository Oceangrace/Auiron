use crate::modules::interfaces::TriggerState;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct WidgetReplayReport {
    states: BTreeMap<TriggerState, u64>,
    transitions: u64,
    #[serde(skip)]
    previous: Option<TriggerState>,
}

impl WidgetReplayReport {
    pub(super) fn observe(&mut self, state: TriggerState) {
        *self.states.entry(state).or_default() += 1;

        if self.previous.is_some_and(|previous| previous != state) {
            self.transitions += 1;
        }

        self.previous = Some(state);
    }
}
