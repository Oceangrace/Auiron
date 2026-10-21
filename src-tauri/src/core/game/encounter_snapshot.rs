use super::EntityId;
use super::NpcDefinitionId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EncounterSnapshot {
    entity_id: EntityId,
    npc_id: NpcDefinitionId,
    boss: bool,
    started_at_us: u64,
}

impl EncounterSnapshot {
    pub(crate) fn new(
        entity_id: EntityId,
        npc_id: NpcDefinitionId,
        boss: bool,
        started_at_us: u64,
    ) -> Self {
        Self {
            entity_id,
            npc_id,
            boss,
            started_at_us,
        }
    }

    pub(crate) fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub(crate) fn npc_id(&self) -> NpcDefinitionId {
        self.npc_id
    }

    pub(crate) fn boss(&self) -> bool {
        self.boss
    }

    pub(crate) fn started_at_us(&self) -> u64 {
        self.started_at_us
    }
}
