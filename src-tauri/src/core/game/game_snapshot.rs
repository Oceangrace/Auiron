use super::EntityId;
use super::EventRecord;
use super::GameCapabilities;
use super::MapId;
use super::SnapshotHeader;
use super::SourceStatus;
use super::TargetObservation;
use super::UnitSnapshot;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GameSnapshot {
    #[serde(flatten)]
    header: SnapshotHeader,
    world: super::WorldSnapshot,
    party: Option<super::PartyRoster>,
    player_id: Option<EntityId>,
    target: TargetObservation,
    map_id: Option<MapId>,
    entities: BTreeMap<EntityId, UnitSnapshot>,
    combat: Option<super::CombatSnapshot>,
    statistics: Vec<super::CombatRecord>,
    cooldowns: Vec<super::CooldownSnapshot>,
    combos: Vec<super::ComboAvailability>,
    skill_book: super::SkillBook,
    events: Vec<EventRecord>,
    capabilities: GameCapabilities,
}

impl GameSnapshot {
    pub(crate) fn new(
        header: SnapshotHeader,
        player_id: Option<EntityId>,
        target: TargetObservation,
        map_id: Option<MapId>,
        entities: BTreeMap<EntityId, UnitSnapshot>,
        events: Vec<EventRecord>,
        capabilities: GameCapabilities,
    ) -> Self {
        Self {
            header,
            world: Default::default(),
            party: None,
            player_id,
            target,
            map_id,
            entities,
            events,
            combat: None,
            statistics: Vec::new(),
            cooldowns: Vec::new(),
            combos: Vec::new(),
            skill_book: Default::default(),
            capabilities,
        }
    }

    pub(crate) fn with_world(mut self, world: super::WorldSnapshot) -> Self {
        self.world = world;
        self
    }

    pub(crate) fn world(&self) -> &super::WorldSnapshot {
        &self.world
    }

    pub(crate) fn with_statistics(mut self, statistics: Vec<super::CombatRecord>) -> Self {
        self.statistics = statistics;
        self
    }

    pub(crate) fn statistics(&self) -> &[super::CombatRecord] {
        &self.statistics
    }

    pub(crate) fn with_combat(mut self, combat: super::CombatSnapshot) -> Self {
        self.combat = Some(combat);
        self
    }

    pub(crate) fn combat(&self) -> Option<&super::CombatSnapshot> {
        self.combat.as_ref()
    }

    pub(crate) fn root_actor(&self, mut id: EntityId) -> Option<EntityId> {
        let mut visited = BTreeSet::new();

        for _ in 0..8 {
            if !visited.insert(id) {
                return None;
            }

            let Some(owner) = self.entities.get(&id).and_then(UnitSnapshot::owner_id) else {
                return Some(id);
            };

            id = owner;
        }

        None
    }

    pub(crate) fn group_member(&self, id: EntityId) -> Option<bool> {
        if self.player_id == Some(id) {
            return Some(true);
        }

        let party = self.party()?;

        if party
            .members()
            .iter()
            .any(|member| member.entity_id() == Some(id))
        {
            return Some(true);
        }

        (party.complete()
            && party
                .members()
                .iter()
                .all(|member| member.entity_id().is_some()))
        .then_some(false)
    }

    pub(crate) fn with_skills(
        mut self,
        cooldowns: Vec<super::CooldownSnapshot>,
        combos: Vec<super::ComboAvailability>,
        skill_book: super::SkillBook,
    ) -> Self {
        self.cooldowns = cooldowns;
        self.combos = combos;
        self.skill_book = skill_book;
        self
    }

    pub(crate) fn cooldowns(&self) -> &[super::CooldownSnapshot] {
        &self.cooldowns
    }

    pub(crate) fn combos(&self) -> &[super::ComboAvailability] {
        &self.combos
    }

    pub(crate) fn skill_book(&self) -> &super::SkillBook {
        &self.skill_book
    }

    pub(crate) fn with_party(mut self, party: Option<super::PartyRoster>) -> Self {
        self.party = party;
        self
    }

    pub(crate) fn party(&self) -> Option<&super::PartyRoster> {
        self.party.as_ref()
    }

    pub(crate) fn capabilities(&self) -> &GameCapabilities {
        &self.capabilities
    }

    pub(crate) fn header(&self) -> &SnapshotHeader {
        &self.header
    }

    pub(crate) fn player_id(&self) -> Option<EntityId> {
        self.player_id
    }

    pub(crate) fn player_in_world(&self) -> bool {
        matches!(
            self.header.status(),
            SourceStatus::Live | SourceStatus::Replay
        ) && self
            .player_id
            .and_then(|id| self.entities.get(&id))
            .is_some_and(UnitSnapshot::present)
    }

    pub(crate) fn target(&self) -> &TargetObservation {
        &self.target
    }

    pub(crate) fn map_id(&self) -> Option<MapId> {
        self.map_id
    }

    pub(crate) fn entities(&self) -> &BTreeMap<EntityId, UnitSnapshot> {
        &self.entities
    }

    pub(crate) fn events(&self) -> &[EventRecord] {
        &self.events
    }
}
