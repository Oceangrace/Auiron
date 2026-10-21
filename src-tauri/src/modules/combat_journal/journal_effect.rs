use crate::core::game::EffectTiming;
use crate::core::game::EntityId;
use crate::core::game::EventRecord;
use crate::core::game::GameEvent;
use crate::core::game::ObservedEffect;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct JournalEffect {
    sequence: u64,
    at_us: u64,
    unit_id: u64,
    instance_id: u64,
    effect_id: Option<u32>,
    skill_id: Option<u32>,
    source_id: Option<u64>,
    duration_ms: Option<u64>,
    #[serde(default)]
    expires_at_us: Option<u64>,
    #[serde(skip)]
    server_end_ms: Option<u64>,
    removed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    removed_by: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    removal_skill: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    removal_reason: Option<u8>,
}

impl JournalEffect {
    pub(super) fn project_since(records: &[EventRecord], after: u64) -> Vec<Self> {
        let clock = records
            .iter()
            .rev()
            .find_map(|record| match record.event() {
                GameEvent::Clock { server_ms } => {
                    Some(i128::from(*server_ms) * 1000 - i128::from(record.at_us()))
                }
                _ => None,
            });

        records
            .iter()
            .filter(|record| record.sequence() > after)
            .flat_map(Self::project)
            .map(|mut effect| {
                effect.expires_at_us = effect
                    .server_end_ms
                    .zip(clock)
                    .and_then(|(end, offset)| u64::try_from(i128::from(end) * 1000 - offset).ok());

                effect
            })
            .collect()
    }

    fn project(record: &EventRecord) -> Vec<Self> {
        match record.event() {
            GameEvent::EffectApplied {
                entity_id, effect, ..
            }
            | GameEvent::EffectUpdated {
                entity_id, effect, ..
            } => vec![Self::active(record, *entity_id, effect)],
            GameEvent::EffectsListed { entity_id, effects } => effects
                .iter()
                .map(|effect| Self::active(record, *entity_id, effect))
                .collect(),
            GameEvent::EffectRemoved {
                entity_id,
                instance_id,
                effect,
                reason,
                removed_by,
                removal_skill,
            } => vec![Self {
                sequence: record.sequence(),
                at_us: record.at_us(),
                unit_id: entity_id.get(),
                instance_id: instance_id.get(),
                effect_id: effect.as_ref().map(|effect| effect.effect_id().get()),
                skill_id: effect
                    .as_ref()
                    .and_then(|effect| effect.skill_id())
                    .map(|id| id.get()),
                source_id: effect
                    .as_ref()
                    .and_then(|effect| effect.source_id())
                    .map(|id| id.get()),
                duration_ms: None,
                expires_at_us: None,
                server_end_ms: None,
                removed: true,
                removed_by: removed_by.map(|id| id.get()),
                removal_skill: removal_skill.map(|id| id.get()),
                removal_reason: Some(*reason),
            }],
            _ => Vec::new(),
        }
    }

    fn active(record: &EventRecord, unit: EntityId, effect: &ObservedEffect) -> Self {
        let (duration_ms, server_end_ms) = match effect.timing() {
            EffectTiming::Untimed => (None, None),
            EffectTiming::Timed {
                duration_ms,
                ends_at_server_ms,
            } => (Some(*duration_ms), *ends_at_server_ms),
        };

        Self {
            sequence: record.sequence(),
            at_us: record.at_us(),
            unit_id: unit.get(),
            instance_id: effect.instance_id().get(),
            effect_id: Some(effect.effect_id().get()),
            skill_id: effect.skill_id().map(|id| id.get()),
            source_id: effect.source_id().map(|id| id.get()),
            duration_ms,
            expires_at_us: None,
            server_end_ms,
            removed: false,
            removed_by: None,
            removal_skill: None,
            removal_reason: None,
        }
    }
}
