use super::EntityId;
use super::SkillId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ActionSnapshot {
    skill_id: SkillId,
    at_us: u64,
    base_casting_ms: Option<u32>,
    base_cooldown_ms: Option<u32>,
    cooldown_group: Option<u32>,
    recipient_id: Option<EntityId>,
}

impl ActionSnapshot {
    pub(crate) fn new(
        skill_id: SkillId,
        at_us: u64,
        casting: Option<u32>,
        cooldown: Option<u32>,
        group: Option<u32>,
    ) -> Self {
        Self {
            skill_id,
            at_us,
            base_casting_ms: casting,
            base_cooldown_ms: cooldown,
            cooldown_group: group,
            recipient_id: None,
        }
    }

    pub(crate) fn skill_id(&self) -> SkillId {
        self.skill_id
    }

    pub(crate) fn at_us(&self) -> u64 {
        self.at_us
    }

    pub(crate) fn casting_ms(&self) -> Option<u32> {
        self.base_casting_ms
    }

    pub(crate) fn cooldown_ms(&self) -> Option<u32> {
        self.base_cooldown_ms
    }

    pub(crate) fn cooldown_group(&self) -> Option<u32> {
        self.cooldown_group
    }

    /// Another entity the skill was aimed at; self-casts and area casts have none.
    pub(crate) fn with_recipient(mut self, recipient: Option<EntityId>) -> Self {
        self.recipient_id = recipient;
        self
    }

    pub(crate) fn recipient_id(&self) -> Option<EntityId> {
        self.recipient_id
    }
}
