use super::EncounterSnapshot;
use super::EntityId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CombatSnapshot {
    active: Option<bool>,
    started_at_us: Option<u64>,
    elapsed_ms: Option<u64>,
    encounter: Option<EncounterSnapshot>,
    opponents: Vec<EntityId>,
}

impl CombatSnapshot {
    pub(crate) fn new(
        active: Option<bool>,
        started_at_us: Option<u64>,
        elapsed_ms: Option<u64>,
        encounter: Option<EncounterSnapshot>,
        opponents: Vec<EntityId>,
    ) -> Self {
        Self {
            active,
            started_at_us,
            elapsed_ms,
            encounter,
            opponents,
        }
    }

    pub(crate) fn active(&self) -> Option<bool> {
        self.active
    }

    pub(crate) fn elapsed_ms(&self) -> Option<u64> {
        self.elapsed_ms
    }

    pub(crate) fn encounter(&self) -> Option<&EncounterSnapshot> {
        self.encounter.as_ref()
    }

    pub(crate) fn opponents(&self) -> &[EntityId] {
        &self.opponents
    }
}
