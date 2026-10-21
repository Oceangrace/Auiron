use super::GameEventType;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GameCapabilities {
    event_types: BTreeSet<GameEventType>,
    effects: bool,
    party: bool,
    profiles: bool,
    npc_definitions: bool,
    maps: bool,
}

impl GameCapabilities {
    pub(crate) fn new(
        event_types: BTreeSet<GameEventType>,
        effects: bool,
        party: bool,
        profiles: bool,
        npc_definitions: bool,
        maps: bool,
    ) -> Self {
        Self {
            event_types,
            effects,
            party,
            profiles,
            npc_definitions,
            maps,
        }
    }

    pub(crate) fn supports(&self, event: GameEventType) -> bool {
        self.event_types.contains(&event)
    }

    pub(crate) fn party(&self) -> bool {
        self.party
    }

    pub(crate) fn effects(&self) -> bool {
        self.effects
    }
}
