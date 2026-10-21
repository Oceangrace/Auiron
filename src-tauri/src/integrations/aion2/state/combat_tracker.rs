use crate::core::game::CombatSnapshot;
use crate::core::game::EncounterSnapshot;
use crate::core::game::EntityId;
use crate::core::game::EventRecord;
use crate::core::game::GameEvent;
use crate::core::game::NpcDefinitionId;
use std::collections::BTreeSet;

pub(super) struct CombatTracker {
    idle_us: u64,
    started: Option<u64>,
    last: Option<u64>,
    ended: Option<u64>,
    encounter: Option<EncounterSnapshot>,
    opponents: BTreeSet<EntityId>,
}

impl CombatTracker {
    pub(super) fn new(idle_ms: u64) -> Self {
        Self {
            idle_us: idle_ms.saturating_mul(1000),
            started: None,
            last: None,
            ended: None,
            encounter: None,
            opponents: BTreeSet::new(),
        }
    }

    pub(super) fn observe(
        &mut self,
        event: &GameEvent,
        at_us: u64,
        player: Option<EntityId>,
        friendly: (bool, bool),
        opponent: Option<(EntityId, NpcDefinitionId, bool)>,
        party_defeated: bool,
    ) -> Vec<EventRecord> {
        let mut events = self.tick(at_us, player);

        if let GameEvent::Died { entity_id } | GameEvent::Despawned { entity_id } = event {
            let encounter = self
                .encounter
                .as_ref()
                .filter(|encounter| encounter.entity_id() == *entity_id);

            if encounter.is_some_and(|encounter| encounter.boss())
                && matches!(event, GameEvent::Died { .. })
            {
                self.end(at_us, player, &mut events);
            } else if encounter.is_some_and(|encounter| !encounter.boss()) {
                self.end_encounter(at_us, &mut events);
            }

            if matches!(event, GameEvent::Died { .. })
                && (party_defeated || (player == Some(*entity_id) && !self.boss_active()))
            {
                self.end(at_us, player, &mut events);
            }

            return events;
        }

        if event.source().is_some() && event.source() == event.recipient() {
            return events;
        }

        // Healing alone does not prove a fight: potions and recovery can be used out of combat.
        if !matches!(
            event,
            GameEvent::Damage { .. }
                | GameEvent::DamageUnknown { .. }
                | GameEvent::Miss { .. }
                | GameEvent::Resist { .. }
        ) || (!friendly.0 && !friendly.1)
        {
            return events;
        }

        let Some(player) = player else {
            return events;
        };

        if self.started.is_none() || self.ended.is_some() {
            self.started = Some(at_us);
            self.ended = None;
            self.opponents.clear();
            events.push(EventRecord::new(
                0,
                at_us,
                GameEvent::CombatStarted { entity_id: player },
            ));
        }

        self.last = Some(at_us);

        if let Some((entity_id, npc_id, boss)) = opponent {
            self.opponents.insert(entity_id);

            let replace = self
                .encounter
                .as_ref()
                .is_none_or(|current| current.entity_id() != entity_id && boss && !current.boss());

            if replace {
                self.end_encounter(at_us, &mut events);
                self.encounter = Some(EncounterSnapshot::new(entity_id, npc_id, boss, at_us));
                events.push(EventRecord::new(
                    0,
                    at_us,
                    GameEvent::EncounterStarted { entity_id, npc_id },
                ));
            }
        }

        events
    }

    pub(super) fn tick(&mut self, now_us: u64, player: Option<EntityId>) -> Vec<EventRecord> {
        let mut events = Vec::new();

        if let Some(last) = self.last
            && self.ended.is_none()
            && !self.boss_active()
            && now_us >= last.saturating_add(self.idle_us)
        {
            self.end(last.saturating_add(self.idle_us), player, &mut events);
        }

        events
    }

    fn boss_active(&self) -> bool {
        self.encounter.as_ref().is_some_and(EncounterSnapshot::boss)
    }

    fn end(&mut self, at_us: u64, player: Option<EntityId>, events: &mut Vec<EventRecord>) {
        if self.started.is_none() || self.ended.is_some() {
            return;
        }

        self.end_encounter(at_us, events);
        self.ended = Some(at_us);

        if let Some(entity_id) = player {
            events.push(EventRecord::new(
                0,
                at_us,
                GameEvent::CombatEnded { entity_id },
            ));
        }
    }

    fn end_encounter(&mut self, at_us: u64, events: &mut Vec<EventRecord>) {
        if let Some(encounter) = self.encounter.take() {
            events.push(EventRecord::new(
                0,
                at_us,
                GameEvent::EncounterEnded {
                    entity_id: encounter.entity_id(),
                    npc_id: encounter.npc_id(),
                },
            ));
        }
    }

    pub(super) fn idle_ms(&self) -> u64 {
        self.idle_us / 1000
    }

    pub(super) fn snapshot(&self, now_us: u64, player: Option<EntityId>) -> CombatSnapshot {
        let active = player.map(|_| self.started.is_some() && self.ended.is_none());

        let elapsed_ms = self
            .started
            .map(|started| self.ended.unwrap_or(now_us).saturating_sub(started) / 1000);

        CombatSnapshot::new(
            active,
            self.started,
            elapsed_ms,
            self.encounter.clone(),
            self.opponents.iter().copied().collect(),
        )
    }
}
