use super::EffectId;
use super::EffectInstanceId;
use super::EffectTiming;
use super::EntityId;
use super::SkillId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ObservedEffect {
    instance_id: EffectInstanceId,
    effect_id: EffectId,
    source_id: Option<EntityId>,
    level: u8,
    skill_id: Option<SkillId>,
    timing: EffectTiming,
}

impl ObservedEffect {
    pub(crate) fn new(
        instance_id: EffectInstanceId,
        effect_id: EffectId,
        source_id: Option<EntityId>,
        level: u8,
        skill_id: Option<SkillId>,
        timing: EffectTiming,
    ) -> Self {
        Self {
            instance_id,
            effect_id,
            source_id,
            level,
            skill_id,
            timing,
        }
    }

    pub(crate) fn instance_id(&self) -> EffectInstanceId {
        self.instance_id
    }

    pub(crate) fn effect_id(&self) -> EffectId {
        self.effect_id
    }

    pub(crate) fn source_id(&self) -> Option<EntityId> {
        self.source_id
    }

    pub(crate) fn level(&self) -> u8 {
        self.level
    }

    pub(crate) fn skill_id(&self) -> Option<SkillId> {
        self.skill_id
    }

    pub(crate) fn timing(&self) -> &EffectTiming {
        &self.timing
    }
}
