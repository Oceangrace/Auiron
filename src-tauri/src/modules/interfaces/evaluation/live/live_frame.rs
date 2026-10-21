use super::trigger_state::TriggerState;
use crate::core::game::GameSnapshot;
use crate::core::game::SourceStatus;
use crate::modules::interfaces::evaluation::PreviewVisual;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LiveFrame {
    status: SourceStatus,
    in_world: bool,
    /// Snapshot time; for a live capture it is Unix time, so the client can subtract frame age.
    at_us: Option<u64>,
    error: Option<String>,
    visuals: HashMap<String, PreviewVisual>,
    states: HashMap<String, TriggerState>,
}

impl LiveFrame {
    pub(super) fn new(status: SourceStatus, error: Option<String>) -> Self {
        Self {
            status,
            in_world: false,
            at_us: None,
            error,
            visuals: HashMap::new(),
            states: HashMap::new(),
        }
    }

    pub(crate) fn states(&self) -> &HashMap<String, TriggerState> {
        &self.states
    }

    pub(super) fn observe_world(&mut self, game: &GameSnapshot) {
        self.at_us = Some(game.header().at_us());
        self.in_world = game.player_in_world();
    }

    pub(crate) fn visuals(&self) -> &HashMap<String, PreviewVisual> {
        &self.visuals
    }

    pub(super) fn insert(
        &mut self,
        id: String,
        state: TriggerState,
        visuals: HashMap<String, PreviewVisual>,
    ) {
        self.states.insert(id, state);
        self.visuals.extend(visuals);
    }
}
