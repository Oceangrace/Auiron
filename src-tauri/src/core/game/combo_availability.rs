use super::SkillId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ComboAvailability {
    skill_id: SkillId,
    available: bool,
    at_us: u64,
}

impl ComboAvailability {
    pub(crate) fn new(skill_id: SkillId, available: bool, at_us: u64) -> Self {
        Self {
            skill_id,
            available,
            at_us,
        }
    }

    pub(crate) fn skill_id(&self) -> SkillId {
        self.skill_id
    }

    pub(crate) fn available(&self) -> bool {
        self.available
    }

    pub(crate) fn at_us(&self) -> u64 {
        self.at_us
    }
}
