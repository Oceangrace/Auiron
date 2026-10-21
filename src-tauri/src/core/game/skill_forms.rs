use super::SkillId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SkillForms {
    skill_id: SkillId,
    forms: Vec<SkillId>,
}

impl SkillForms {
    pub(crate) fn new(skill_id: SkillId, forms: Vec<SkillId>) -> Self {
        Self { skill_id, forms }
    }
}
