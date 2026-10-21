use super::EntityId;
use super::SkillId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CastPreparation {
    skill_id: SkillId,
    target_id: Option<EntityId>,
    started_at_us: u64,
    duration_ms: u32,
}

impl CastPreparation {
    pub(crate) fn new(
        skill_id: SkillId,
        target_id: Option<EntityId>,
        started_at_us: u64,
        duration_ms: u32,
    ) -> Self {
        Self {
            skill_id,
            target_id,
            started_at_us,
            duration_ms,
        }
    }

    pub(crate) fn skill_id(&self) -> SkillId {
        self.skill_id
    }

    pub(crate) fn target_id(&self) -> Option<EntityId> {
        self.target_id
    }

    pub(crate) fn started_at_us(&self) -> u64 {
        self.started_at_us
    }

    pub(crate) fn duration_ms(&self) -> u32 {
        self.duration_ms
    }

    pub(crate) fn remaining_ms(&self, now_us: u64) -> f64 {
        let elapsed = now_us.saturating_sub(self.started_at_us) as f64 / 1000.0;

        (f64::from(self.duration_ms) - elapsed).max(0.0)
    }
}
