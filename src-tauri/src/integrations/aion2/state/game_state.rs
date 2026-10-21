use super::npc_recovery::NpcRecovery;
use super::player_recovery::PlayerRecovery;
use super::self_tracker::SelfTracker;
use super::server_clock::ServerClock;
use super::unit_state::UnitState;
use crate::core::game::ActionSnapshot;
use crate::core::game::EntityId;
use crate::core::game::EventRecord;
use crate::core::game::GameCapabilities;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::core::game::MapId;
use crate::core::game::SkillBook;
use crate::core::game::SkillId;
use crate::core::game::SnapshotHeader;
use crate::core::game::SourceStatus;
use crate::core::game::TargetObservation;
use crate::core::game::WorldBossList;
use crate::integrations::aion2::StreamReset;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

pub(crate) struct GameState {
    catalog: Arc<RuntimeCatalog>,
    observation_id: String,
    capabilities: GameCapabilities,
    revision: u64,
    sequence: u64,
    clock: ServerClock,
    player: SelfTracker,
    skills: super::skill_tracker::SkillTracker,
    // Character state: unlike cooldowns, it survives map changes without a new list.
    skill_book: SkillBook,
    combat: super::combat_tracker::CombatTracker,
    party: super::party_tracker::PartyTracker,
    world: super::world_tracker::WorldTracker,
    ledger: super::combat_ledger::CombatLedger,
    target: TargetObservation,
    // A selection refused because the cached unit looked dead: an entity ID reused before
    // its new spawn was seen. It becomes the target once the unit shows it is alive.
    pending_target: Option<EntityId>,
    // Set when this observation follows a capture gap; reported in every snapshot.
    interrupted: bool,
    map_id: Option<MapId>,
    entities: BTreeMap<EntityId, UnitState>,
    journal: VecDeque<EventRecord>,
}

impl GameState {
    /// Events kept for consumers between snapshots; journal cursors detect overflow.
    const JOURNAL_EVENTS: usize = 512;
    const JOURNAL_AGE_US: u64 = 60_000_000;
    const MAX_ENTITIES: usize = 4096;

    pub(crate) fn new(
        observation_id: String,
        capabilities: GameCapabilities,
        catalog: Arc<RuntimeCatalog>,
        combat_idle_ms: u64,
    ) -> Self {
        Self {
            observation_id,
            catalog,
            capabilities,
            revision: 0,
            sequence: 0,
            clock: ServerClock::new(),
            player: SelfTracker::new(),
            skills: Default::default(),
            skill_book: Default::default(),
            combat: super::combat_tracker::CombatTracker::new(combat_idle_ms),
            party: Default::default(),
            world: Default::default(),
            ledger: Default::default(),
            target: TargetObservation::Unknown,
            pending_target: None,
            interrupted: false,
            map_id: None,
            entities: BTreeMap::new(),
            journal: VecDeque::new(),
        }
    }

    pub(crate) fn reset(&mut self, observation_id: String) {
        *self = Self::new(
            observation_id,
            self.capabilities.clone(),
            self.catalog.clone(),
            self.combat.idle_ms(),
        );
    }

    pub(crate) fn restore_player(&mut self, recovery: &PlayerRecovery) -> Result<(), String> {
        self.apply(&recovery.identity(), 0)?;
        self.skill_book = recovery.skills().clone();

        if let Some(id) = self.player.id() {
            self.entity(id)?
                .restore_resources(recovery.health(), recovery.mana());
        }

        Ok(())
    }

    pub(crate) fn restore_bosses(&mut self, lists: &[WorldBossList]) {
        for list in lists.iter().take(32) {
            if list.bosses().len() <= 200 {
                self.world
                    .observe(&GameEvent::WorldBossList { list: list.clone() });
            }
        }
    }

    pub(crate) fn restore_npcs(
        &mut self,
        map_id: Option<MapId>,
        npcs: &[NpcRecovery],
        now_us: u64,
    ) -> Result<(), String> {
        if self.map_id.is_some() && self.map_id != map_id {
            return Ok(());
        }

        self.map_id = self.map_id.or(map_id);

        for npc in npcs.iter().take(Self::MAX_ENTITIES / 2) {
            if npc.valid(now_us) && !self.entities.contains_key(&npc.entity_id()) {
                self.entity(npc.entity_id())?.restore_npc(npc);
            }
        }

        Ok(())
    }

    pub(crate) fn has_player(&self) -> bool {
        self.player
            .id()
            .is_some_and(|id| self.entities.contains_key(&id))
    }

    fn reconcile_player(&mut self, event: &GameEvent) {
        if self.player.id().is_none() {
            return;
        }

        let changed = match event {
            GameEvent::SelfIdentified {
                entity_id,
                authoritative: true,
            } => self.player.id() != Some(*entity_id),

            GameEvent::Identity {
                entity_id,
                own,
                name,
                server_id,
                class_name,
                ..
            } => {
                let is_player = self.player.id() == Some(*entity_id);

                (*own && !is_player)
                    || (is_player
                        && self.entities.get(entity_id).is_some_and(|unit| {
                            let identity = unit.identity_info();

                            name.as_deref()
                                .zip(identity.name())
                                .is_some_and(|(new, old)| new != old)
                                || server_id
                                    .zip(identity.server_id())
                                    .is_some_and(|(new, old)| new != old)
                                || class_name
                                    .zip(identity.class_name())
                                    .is_some_and(|(new, old)| new != old)
                        }))
            }

            _ => false,
        };

        if changed {
            self.reset(uuid::Uuid::new_v4().to_string());
        }
    }

    pub(crate) fn recover(&mut self, reset: &StreamReset) {
        let observation_id = uuid::Uuid::new_v4().to_string();

        match reset {
            StreamReset::Closed => self.reset(observation_id),
            StreamReset::Interrupted(_) => self.interrupt(observation_id),
        }
    }

    /// A capture gap loses events, not the character: identity and the skill book arrive
    /// only on world entry, so they survive while every observed unit becomes unknown.
    fn interrupt(&mut self, observation_id: String) {
        let player = std::mem::replace(&mut self.player, SelfTracker::new());

        let skill_book = std::mem::take(&mut self.skill_book);

        self.reset(observation_id);
        self.player = player;
        self.skill_book = skill_book;
        self.interrupted = true;
    }

    pub(crate) fn apply(&mut self, event: &GameEvent, at_us: u64) -> Result<(), String> {
        self.reconcile_player(event);

        if let GameEvent::SelfRestoration {
            entity_id,
            skill_id,
            value,
            hit,
        } = event
        {
            let known_player = self.player.id() == Some(*entity_id)
                || self.entities.get(entity_id).is_some_and(|unit| {
                    unit.npc_id().is_none()
                        && (unit.identity_info().character_id().is_some()
                            || unit.identity_info().class_name().is_some())
                });

            if known_player {
                return self.apply(
                    &GameEvent::Healing {
                        entity_id: *entity_id,
                        source_id: *entity_id,
                        skill_id: *skill_id,
                        value: *value,
                        hit: Some(hit.clone()),
                    },
                    at_us,
                );
            }

            return Ok(());
        }

        self.world.observe(event);

        let journal = match event {
            GameEvent::HealingPool { .. } | GameEvent::SkillConfirmed { .. } => true,
            GameEvent::PropertiesChanged {
                entity_id,
                health,
                mana,
            } => self.entity(*entity_id)?.observe_resources(health, mana),
            GameEvent::ComboAvailable { .. } | GameEvent::ComboUnavailable { .. } => {
                self.player.id().is_some()
            }

            _ => event.event_type().is_some(),
        };

        let journal_event = journal.then(|| {
            let observed = if let GameEvent::EffectRemoved {
                entity_id,
                instance_id,
                ..
            } = event
            {
                self.entities
                    .get(entity_id)
                    .and_then(|unit| unit.effect(*instance_id))
                    .cloned()
            } else {
                None
            };

            event.clone().with_removed_effect(observed)
        });

        match event {
            GameEvent::TargetSelected { entity_id } => {
                self.pending_target = *entity_id;
                self.target = match entity_id {
                    Some(id)
                        if self
                            .entities
                            .get(id)
                            .is_none_or(|unit| unit.current_health() != Some(0)) =>
                    {
                        TargetObservation::Selected(*id)
                    }
                    _ => TargetObservation::Cleared,
                };
            }

            GameEvent::CharacterIdentified {
                entity_id,
                character_id,
                level,
            } => self
                .entity(*entity_id)?
                .identify_character(*character_id, *level),
            GameEvent::CastStarted {
                source_id,
                preparation,
            } => self.entity(*source_id)?.begin_cast(preparation.clone()),
            GameEvent::CooldownChanged {
                group_id,
                remaining_ms,
            } => self.skills.remaining(*group_id, *remaining_ms, at_us)?,
            GameEvent::SkillConfirmed { skill_id } => {
                self.skills.used(*skill_id, at_us, true, &self.catalog)?
            }

            GameEvent::ComboAvailable { skill_id, .. } => {
                self.skills.combo(*skill_id, true, at_us)?
            }

            GameEvent::ComboUnavailable { skill_id } => {
                self.skills.combo(*skill_id, false, at_us)?
            }

            GameEvent::SkillsLearned { skills } => self.skill_book.learn(skills.clone()),
            GameEvent::SkillLevelChanged { skill } => self.skill_book.update(skill.clone()),
            GameEvent::SkillFormsListed { skills } => self.skill_book.select_forms(skills.clone()),

            GameEvent::PropertiesChanged {
                entity_id,
                health,
                mana,
            } => {
                let unit = self.entity(*entity_id)?;

                unit.health(health.current(), health.maximum(), at_us);
                unit.mana(mana);
            }

            GameEvent::Party { roster } => self.party.observe(roster.clone()),
            GameEvent::Attribution {
                entity_id,
                actor_id,
            } => {
                let summon = self
                    .entities
                    .get(entity_id)
                    .and_then(UnitState::npc_id)
                    .and_then(|id| self.catalog.npc(id.get()))
                    .is_some_and(|npc| npc.player_summon());

                if summon {
                    self.entity(*entity_id)?.owner(*actor_id);
                }
            }

            GameEvent::ServerBattleChanged { entity_id, battle } => {
                self.entity(*entity_id)?.server_battle(battle.clone())
            }

            GameEvent::Clock { server_ms } => self.clock.sync(*server_ms, at_us),
            GameEvent::MapChanged { map_id } => {
                self.map_id = Some(*map_id);
                self.target = TargetObservation::Unknown;
                self.journal.clear();
                self.skills = Default::default();
                self.ledger = Default::default();
                self.combat = super::combat_tracker::CombatTracker::new(self.combat.idle_ms());
                self.observation_id = uuid::Uuid::new_v4().to_string();
                // A map change is a clean boundary, not a continuation after a gap.
                self.interrupted = false;

                let own = self.player.id();

                self.entities.retain(|id, _| Some(*id) == own);

                for entity in self.entities.values_mut() {
                    entity.travel();
                }
            }

            GameEvent::SelfIdentified {
                entity_id,
                authoritative,
            } => {
                self.entity(*entity_id)?;
                self.player.identify(*entity_id, *authoritative);
            }

            GameEvent::Identity {
                entity_id,
                name,
                own,
                server_id,
                class_name,
                level,
            } => {
                if *own {
                    self.player.identify(*entity_id, true);
                }

                self.entity(*entity_id)?
                    .identity(name, *class_name, *server_id, *level);
            }

            GameEvent::Spawned {
                entity_id,
                npc_id,
                health,
                ..
            } => self
                .entity(*entity_id)?
                .spawn(*npc_id, health.as_ref(), at_us),
            GameEvent::EffectApplied {
                entity_id,
                effect,
                restarted,
            }
            | GameEvent::EffectUpdated {
                entity_id,
                effect,
                restarted,
            } => {
                let limit = self
                    .catalog
                    .effect(effect.effect_id().get())
                    .filter(|_| matches!(event, GameEvent::EffectApplied { .. }))
                    .map(|metadata| metadata.stack_limit());

                self.entity(*entity_id)?
                    .put_effect(effect.clone(), at_us, *restarted, limit)?;
            }

            GameEvent::EffectsListed { entity_id, effects } => {
                self.entity(*entity_id)?.list_effects(effects, at_us)?
            }

            GameEvent::EffectRemoved {
                entity_id,
                instance_id,
                ..
            } => self.entity(*entity_id)?.remove_effect(*instance_id),
            GameEvent::SkillUsed {
                entity_id,
                source_id,
                skill_id,
            } => {
                let known_recipient = entity_id.is_some_and(|id| self.entities.contains_key(&id));

                let metadata = self.catalog.skill(skill_id.get());

                let action = ActionSnapshot::new(
                    *skill_id,
                    at_us,
                    metadata.map(|skill| skill.casting_ms()),
                    metadata.map(|skill| skill.cooldown_ms()),
                    metadata.and_then(|skill| skill.cooldown_group()),
                )
                .with_recipient(entity_id.filter(|id| id != source_id));

                let charge_skill = metadata
                    .and_then(|skill| skill.charge_skill())
                    .map(SkillId::new)
                    .transpose()?;

                let unit = self.entity(*source_id)?;

                if let Some(charge_skill) = charge_skill {
                    unit.finish_cast(charge_skill);
                }

                unit.action(action);

                if self.player.id() == Some(*source_id) {
                    self.skills.used(*skill_id, at_us, false, &self.catalog)?;
                }

                if known_recipient
                    && matches!(
                        self.target,
                        TargetObservation::Unknown | TargetObservation::LastSkillRecipient(_)
                    )
                    && self.player.id() == Some(*source_id)
                    && let Some(entity_id) = entity_id
                    && entity_id != source_id
                {
                    self.target = TargetObservation::LastSkillRecipient(*entity_id);
                }
            }

            GameEvent::Damage {
                entity_id,
                source_id,
                ..
            }
            | GameEvent::Healing {
                entity_id,
                source_id,
                ..
            }
            | GameEvent::HitLanded {
                entity_id,
                source_id,
                ..
            }
            | GameEvent::DamageUnknown {
                entity_id,
                source_id,
                ..
            }
            | GameEvent::Miss {
                entity_id,
                source_id,
                ..
            }
            | GameEvent::Resist {
                entity_id,
                source_id,
                ..
            } => {
                self.entity(*entity_id)?;
                self.entity(*source_id)?;
            }

            GameEvent::Died { entity_id } => {
                self.entity(*entity_id)?.health(Some(0), None, at_us);

                // The body may stay until despawn; a dead recipient is no longer the target.
                if self.target.entity_id() == Some(*entity_id) {
                    self.clear_target();
                }
            }

            GameEvent::Despawned { entity_id } => {
                self.entity(*entity_id)?.despawn();

                if self.pending_target == Some(*entity_id) {
                    self.pending_target = None;
                }

                if self.target.entity_id() == Some(*entity_id) {
                    self.clear_target();
                }
            }

            _ => {}
        }

        self.confirm_pending_target();

        let own_alive = self
            .player
            .id()
            .and_then(|id| self.entities.get(&id))
            .is_none_or(|unit| unit.current_health() != Some(0));

        let source_friendly =
            self.friendly(event.source()) && (own_alive || event.source() != self.player.id());

        let recipient_friendly = self.friendly(event.recipient())
            && (own_alive || event.recipient() != self.player.id());

        let candidate = if source_friendly && !recipient_friendly {
            event.recipient()
        } else if recipient_friendly && !source_friendly {
            event.source()
        } else {
            None
        };

        let opponent = candidate
            .and_then(|id| {
                self.entities
                    .get(&id)
                    .filter(|unit| unit.current_health() != Some(0))
                    .and_then(UnitState::npc_id)
                    .map(|npc| (id, npc))
            })
            .and_then(|(id, npc)| {
                self.catalog
                    .npc(npc.get())
                    .map(|traits| (id, npc, traits.boss()))
            });

        let derived = self.combat.observe(
            event,
            at_us,
            self.player.id(),
            (source_friendly, recipient_friendly),
            opponent,
            matches!(event, GameEvent::Died { .. })
                && self.party.defeated(self.player.id(), &self.entities),
        );

        if derived
            .iter()
            .any(|record| matches!(record.event(), GameEvent::CombatStarted { .. }))
        {
            self.ledger = Default::default();
        }

        let ends_now = matches!(event, GameEvent::Died { .. })
            && derived.iter().any(|record| {
                record.at_us() == at_us && matches!(record.event(), GameEvent::CombatEnded { .. })
            });

        if self.combat.snapshot(at_us, self.player.id()).active() == Some(true) || ends_now {
            self.ledger.observe(event, at_us);
        }

        for record in derived {
            self.append(record.event().clone(), record.at_us());
        }

        if let Some(observed) = journal_event {
            self.append(observed, at_us);
        }

        Ok(())
    }

    fn confirm_pending_target(&mut self) {
        let Some(id) = self.pending_target else {
            return;
        };

        if !matches!(self.target, TargetObservation::Cleared) {
            self.pending_target = None;

            return;
        }

        if self
            .entities
            .get(&id)
            .is_some_and(|unit| unit.present() && unit.current_health() != Some(0))
        {
            self.target = TargetObservation::Selected(id);
            self.pending_target = None;
        }
    }

    fn clear_target(&mut self) {
        self.target = match self.target {
            TargetObservation::Selected(_) | TargetObservation::Cleared => {
                TargetObservation::Cleared
            }
            _ => TargetObservation::Unknown,
        };
    }

    fn friendly(&self, id: Option<EntityId>) -> bool {
        let Some(mut id) = id else {
            return false;
        };

        for _ in 0..8 {
            if self.player.id() == Some(id) || self.party.contains(id, &self.entities) {
                return true;
            }

            let Some(owner) = self.entities.get(&id).and_then(UnitState::owner_id) else {
                return false;
            };

            id = owner;
        }

        false
    }

    fn append(&mut self, event: GameEvent, at_us: u64) {
        self.sequence += 1;
        self.journal
            .push_back(EventRecord::new(self.sequence, at_us, event));

        while self.journal.len() > Self::JOURNAL_EVENTS {
            self.journal.pop_front();
        }
    }

    pub(crate) fn snapshot(&mut self, now_us: u64, status: SourceStatus) -> GameSnapshot {
        if let Some(id) = self.player.id()
            && let Some(player) = self.entities.get_mut(&id)
        {
            player.observe_class(&self.catalog);
        }

        for record in self.combat.tick(now_us, self.player.id()) {
            self.append(record.event().clone(), record.at_us());
        }

        self.revision += 1;

        while self
            .journal
            .front()
            .is_some_and(|event| now_us.saturating_sub(event.at_us()) > Self::JOURNAL_AGE_US)
        {
            self.journal.pop_front();
        }

        let entities = self
            .entities
            .iter()
            .map(|(id, entity)| {
                (
                    *id,
                    entity.snapshot(*id, &self.clock, now_us, &self.catalog),
                )
            })
            .collect();

        let header =
            SnapshotHeader::new(self.observation_id.clone(), self.revision, now_us, status)
                .with_interrupted(self.interrupted);

        let party = self.party.snapshot(&entities);

        GameSnapshot::new(
            header,
            self.player.id(),
            self.target.clone(),
            self.map_id,
            entities,
            self.journal.iter().cloned().collect(),
            self.capabilities.clone(),
        )
        .with_world(self.world.snapshot())
        .with_party(party)
        .with_skills(
            self.skills.cooldowns(now_us),
            self.skills.combos(now_us, &self.catalog),
            self.skill_book.clone(),
        )
        .with_combat(self.combat.snapshot(now_us, self.player.id()))
        .with_statistics(self.ledger.snapshot())
    }

    fn entity(&mut self, id: EntityId) -> Result<&mut UnitState, String> {
        if self.entities.len() >= Self::MAX_ENTITIES && !self.entities.contains_key(&id) {
            // Units leave only on map change; evict a despawned one instead of failing the source.
            let keep = [self.player.id(), self.target.entity_id()];

            let stale = self
                .entities
                .iter()
                .find(|(id, unit)| !unit.present() && !keep.contains(&Some(**id)))
                .map(|(id, _)| *id)
                .ok_or("Observed entity limit")?;

            self.entities.remove(&stale);
        }

        Ok(self.entities.entry(id).or_insert_with(UnitState::new))
    }
}
