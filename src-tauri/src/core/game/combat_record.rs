use super::CombatStatistics;
use super::EntityId;
use super::SkillId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CombatRecord {
    source_id: EntityId,
    recipient_id: EntityId,
    skill_id: Option<SkillId>,
    statistics: CombatStatistics,
}

impl CombatRecord {
    pub(crate) fn new(
        source_id: EntityId,
        recipient_id: EntityId,
        skill_id: Option<SkillId>,
        statistics: CombatStatistics,
    ) -> Self {
        Self {
            source_id,
            recipient_id,
            skill_id,
            statistics,
        }
    }

    pub(crate) fn source_id(&self) -> EntityId {
        self.source_id
    }

    pub(crate) fn recipient_id(&self) -> EntityId {
        self.recipient_id
    }

    pub(crate) fn skill_id(&self) -> Option<SkillId> {
        self.skill_id
    }

    pub(crate) fn statistics(&self) -> &CombatStatistics {
        &self.statistics
    }
}
