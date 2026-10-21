use super::LearnedSkill;
use super::SkillForms;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SkillBook {
    learned: Vec<LearnedSkill>,
    complete: bool,
    forms: Option<Vec<SkillForms>>,
}

impl SkillBook {
    pub(crate) fn complete(&self) -> bool {
        self.complete
    }

    pub(crate) fn learn(&mut self, skills: Vec<LearnedSkill>) {
        self.learned = skills;
        self.complete = true;
    }

    pub(crate) fn update(&mut self, skill: LearnedSkill) {
        self.learned
            .retain(|known| known.skill_id() != skill.skill_id());
        self.learned.push(skill);
    }

    pub(crate) fn select_forms(&mut self, skills: Vec<SkillForms>) {
        self.forms = Some(skills);
    }

    pub(crate) fn learned(&self, ids: &[u32]) -> Option<Option<&LearnedSkill>> {
        let skill = self
            .learned
            .iter()
            .filter(|skill| ids.contains(&skill.skill_id().get()))
            .max_by_key(|skill| skill.level());

        if skill.is_some() || self.complete {
            return Some(skill);
        }

        None
    }
}
