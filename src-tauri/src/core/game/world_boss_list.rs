use super::MapId;
use super::WorldBoss;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorldBossList {
    map_id: MapId,
    observed_at_us: u64,
    bosses: Vec<WorldBoss>,
}

impl WorldBossList {
    pub(crate) fn new(map_id: MapId, observed_at_us: u64, bosses: Vec<WorldBoss>) -> Self {
        Self {
            map_id,
            observed_at_us,
            bosses,
        }
    }

    pub(crate) fn map_id(&self) -> MapId {
        self.map_id
    }

    pub(crate) fn observed_at_us(&self) -> u64 {
        self.observed_at_us
    }

    pub(crate) fn bosses(&self) -> &[WorldBoss] {
        &self.bosses
    }
}
