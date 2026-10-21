use super::InstanceTimer;
use super::OdyleEnergy;
use super::WorldBossList;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorldSnapshot {
    energy: Option<OdyleEnergy>,
    timers: Vec<InstanceTimer>,
    boss_lists: Vec<WorldBossList>,
}

impl WorldSnapshot {
    pub(crate) fn new(
        energy: Option<OdyleEnergy>,
        timers: Vec<InstanceTimer>,
        boss_lists: Vec<WorldBossList>,
    ) -> Self {
        Self {
            energy,
            timers,
            boss_lists,
        }
    }

    pub(crate) fn energy(&self) -> Option<&OdyleEnergy> {
        self.energy.as_ref()
    }

    pub(crate) fn timers(&self) -> &[InstanceTimer] {
        &self.timers
    }

    pub(crate) fn boss_lists(&self) -> &[WorldBossList] {
        &self.boss_lists
    }
}
