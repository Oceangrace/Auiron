use super::journal_action::JournalAction;
use super::journal_effect::JournalEffect;
use super::journal_resource::JournalResource;
use super::journal_unit::JournalUnit;
use crate::core::game::CombatRecord;
use crate::core::game::CombatSnapshot;
use crate::core::game::EntityId;
use crate::core::game::GameEvent;
use crate::core::game::GameSnapshot;
use crate::core::game::MapId;
use crate::core::game::SourceFrame;
use crate::core::game::SourceStatus;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JournalFrame {
    observation_id: String,
    at_us: u64,
    status: SourceStatus,
    interrupted: bool,
    error: Option<String>,
    player_id: Option<EntityId>,
    map_id: Option<MapId>,
    dungeon_id: Option<u32>,
    combat: Option<CombatSnapshot>,
    units: Vec<JournalUnit>,
    records: Vec<CombatRecord>,
    actions: Vec<JournalAction>,
    effects: Vec<JournalEffect>,
    resources: Vec<JournalResource>,
    event_sequence: Option<u64>,
    first_event_sequence: Option<u64>,
    first_event_at_us: Option<u64>,
}

impl JournalFrame {
    pub(crate) fn new(source: &SourceFrame, catalog: &RuntimeCatalog) -> Self {
        Self::incremental(source, None, None, catalog)
    }

    pub(crate) fn incremental(
        source: &SourceFrame,
        observation: Option<&str>,
        sequence: Option<u64>,
        catalog: &RuntimeCatalog,
    ) -> Self {
        let snapshot = source.snapshot();
        let after = Self::cursor(snapshot, observation, sequence);

        Self {
            observation_id: snapshot.header().observation_id().into(),
            at_us: snapshot.header().at_us(),
            status: snapshot.header().status(),
            interrupted: snapshot.header().interrupted(),
            error: source.error().map(str::to_owned),
            player_id: snapshot.player_id(),
            map_id: snapshot.map_id(),
            dungeon_id: snapshot.party().and_then(|party| party.dungeon_id()),
            combat: snapshot.combat().cloned(),
            units: snapshot
                .entities()
                .values()
                .map(|unit| JournalUnit::new(unit, snapshot))
                .collect(),
            records: if after == 0 || Self::statistics_changed(snapshot, after) {
                snapshot.statistics().to_vec()
            } else {
                Vec::new()
            },
            actions: snapshot
                .events()
                .iter()
                .filter(|event| event.sequence() > after)
                .filter_map(|record| JournalAction::new(record, catalog))
                .collect(),
            effects: JournalEffect::project_since(snapshot.events(), after),
            resources: snapshot
                .events()
                .iter()
                .filter(|event| event.sequence() > after)
                .filter_map(JournalResource::new)
                .collect(),
            event_sequence: snapshot.events().last().map(|event| event.sequence()),
            first_event_sequence: snapshot.events().first().map(|event| event.sequence()),
            first_event_at_us: snapshot.events().first().map(|event| event.at_us()),
        }
    }

    fn cursor(snapshot: &GameSnapshot, observation: Option<&str>, sequence: Option<u64>) -> u64 {
        if observation != Some(snapshot.header().observation_id()) {
            return 0;
        }

        let current = snapshot.events().last().map_or(0, |event| event.sequence());

        sequence
            .filter(|sequence| {
                *sequence <= current
                    && sequence.saturating_add(1)
                        >= snapshot
                            .events()
                            .first()
                            .map_or(0, |event| event.sequence())
            })
            .unwrap_or(0)
    }

    fn statistics_changed(snapshot: &GameSnapshot, after: u64) -> bool {
        snapshot.events().iter().any(|record| {
            record.sequence() > after
                && matches!(
                    record.event(),
                    GameEvent::Damage { .. }
                        | GameEvent::Healing { .. }
                        | GameEvent::DamageUnknown { .. }
                        | GameEvent::Miss { .. }
                        | GameEvent::Resist { .. }
                        | GameEvent::Died { .. }
                        | GameEvent::SelfRestoration { .. }
                )
        })
    }
}
