use super::EntityId;
use super::UnitIdentity;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PartyMember {
    slot: u8,
    database_id: super::CharacterId,
    identity: UnitIdentity,
    entity_id: Option<EntityId>,
    gear_score: u32,
    combat_power: u64,
}

impl PartyMember {
    pub(crate) fn new(
        slot: u8,
        database_id: super::CharacterId,
        mut identity: UnitIdentity,
        gear_score: u32,
        combat_power: u64,
    ) -> Self {
        identity.identify_character(database_id);

        Self {
            slot,
            database_id,
            identity,
            entity_id: None,
            gear_score,
            combat_power,
        }
    }

    pub(crate) fn with_entity(mut self, entity_id: Option<EntityId>) -> Self {
        self.entity_id = entity_id;
        self
    }

    pub(crate) fn database_id(&self) -> super::CharacterId {
        self.database_id
    }

    pub(crate) fn slot(&self) -> u8 {
        self.slot
    }

    pub(crate) fn identity(&self) -> &UnitIdentity {
        &self.identity
    }

    pub(crate) fn entity_id(&self) -> Option<EntityId> {
        self.entity_id
    }

    pub(crate) fn gear_score(&self) -> u32 {
        self.gear_score
    }

    pub(crate) fn combat_power(&self) -> u64 {
        self.combat_power
    }
}
