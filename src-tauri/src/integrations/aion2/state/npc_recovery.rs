use crate::core::game::EntityId;
use crate::core::game::GameSnapshot;
use crate::core::game::HealthSnapshot;
use crate::core::game::NpcDefinitionId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct NpcRecovery {
    entity_id: EntityId,
    npc_id: NpcDefinitionId,
    health: HealthSnapshot,
}

impl NpcRecovery {
    // Short restarts may miss spawn packets. Do not retain old instances indefinitely.
    const MAX_AGE_US: u64 = 15 * 60 * 1_000_000;
    const MAX_UNITS: usize = 2048;

    pub(crate) fn capture(game: &GameSnapshot) -> Vec<Self> {
        game.entities()
            .values()
            .filter(|unit| unit.present() && unit.owner_id().is_none())
            .filter_map(|unit| {
                let recovery = Self {
                    entity_id: unit.id(),
                    npc_id: unit.npc_id()?,
                    health: unit.health().clone(),
                };

                recovery.valid(game.header().at_us()).then_some(recovery)
            })
            .take(Self::MAX_UNITS)
            .collect()
    }

    pub(crate) fn valid(&self, now_us: u64) -> bool {
        let observed = self.health.observed_at_us();

        observed > 0
            && observed <= now_us
            && now_us - observed <= Self::MAX_AGE_US
            && self
                .health
                .current()
                .zip(self.health.maximum())
                .is_some_and(|(current, maximum)| current > 0 && current <= maximum)
    }

    pub(crate) fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub(crate) fn npc_id(&self) -> NpcDefinitionId {
        self.npc_id
    }

    pub(crate) fn health(&self) -> &HealthSnapshot {
        &self.health
    }
}
