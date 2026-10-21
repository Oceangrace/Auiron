use super::SkillId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LearnedSkill {
    skill_id: SkillId,
    level: u8,
    base_level: u8,
}

impl LearnedSkill {
    pub(crate) fn new(skill_id: SkillId, level: u8, base_level: u8) -> Self {
        Self {
            skill_id,
            level,
            base_level,
        }
    }

    pub(crate) fn skill_id(&self) -> SkillId {
        self.skill_id
    }

    pub(crate) fn level(&self) -> u8 {
        self.level
    }

    pub(crate) fn base_level(&self) -> u8 {
        self.base_level
    }
}
