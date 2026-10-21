use super::combat_accumulator::CombatAccumulator;
use crate::core::game::CombatCounter;
use crate::core::game::CombatRecord;
use crate::core::game::CombatStatistics;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::SkillId;
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct CombatLedger {
    records: BTreeMap<(EntityId, EntityId, Option<SkillId>), CombatStatistics>,
    truncated: bool,
}

impl CombatLedger {
    const MAX_RECORDS: usize = 8192;

    pub(super) fn observe(&mut self, event: &GameEvent, at_us: u64) {
        if let GameEvent::Died { entity_id } = event {
            let key = (*entity_id, *entity_id, None);

            if self.records.len() >= Self::MAX_RECORDS && !self.records.contains_key(&key) {
                self.truncated = true;

                return;
            }

            if !self.records.is_empty() {
                let statistics = self.records.entry(key).or_default();

                statistics.increment(CombatCounter::Deaths, 1);
                statistics.touch(at_us);
            }

            return;
        }

        if !matches!(
            event,
            GameEvent::Damage { .. }
                | GameEvent::Healing { .. }
                | GameEvent::DamageUnknown { .. }
                | GameEvent::Miss { .. }
                | GameEvent::Resist { .. }
        ) {
            return;
        }

        let Some((source, recipient)) = event.source().zip(event.recipient()) else {
            return;
        };

        let key = (source, recipient, event.skill_id());

        if self.records.len() >= Self::MAX_RECORDS && !self.records.contains_key(&key) {
            self.truncated = true;

            return;
        }

        CombatAccumulator::observe(self.records.entry(key).or_default(), event, at_us);
    }

    pub(super) fn snapshot(&self) -> Vec<CombatRecord> {
        self.records
            .iter()
            .map(|((source, recipient, skill), statistics)| {
                let mut statistics = statistics.clone();

                if self.truncated {
                    statistics.incomplete();
                }

                CombatRecord::new(*source, *recipient, *skill, statistics)
            })
            .collect()
    }
}
