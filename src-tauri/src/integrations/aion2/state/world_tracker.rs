use crate::core::game::GameEvent;
use crate::core::game::InstanceTimer;
use crate::core::game::MapId;
use crate::core::game::OdyleEnergy;
use crate::core::game::WorldBossList;
use crate::core::game::WorldSnapshot;
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct WorldTracker {
    energy: Option<OdyleEnergy>,
    timers: BTreeMap<(MapId, u8), InstanceTimer>,
    boss_lists: BTreeMap<MapId, WorldBossList>,
}

impl WorldTracker {
    pub(super) fn observe(&mut self, event: &GameEvent) {
        match event {
            GameEvent::OdyleEnergy { energy } => self.energy = Some(energy.clone()),
            GameEvent::InstanceTimer { timer } => {
                if self.timers.len() < 256
                    || self.timers.contains_key(&(timer.map_id(), timer.kind()))
                {
                    self.timers
                        .insert((timer.map_id(), timer.kind()), timer.clone());
                }
            }

            GameEvent::WorldBossList { list } => {
                if self.boss_lists.len() < 32 || self.boss_lists.contains_key(&list.map_id()) {
                    self.boss_lists.insert(list.map_id(), list.clone());
                }
            }

            GameEvent::MapChanged { map_id } => {
                self.timers.retain(|(map, _), _| map == map_id);
            }

            _ => {}
        }
    }

    pub(super) fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot::new(
            self.energy.clone(),
            self.timers.values().cloned().collect(),
            self.boss_lists.values().cloned().collect(),
        )
    }
}
