use super::CooldownBasis;
use super::CooldownGroupId;
use super::SkillId;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CooldownSnapshot {
    group_id: CooldownGroupId,
    skill_id: Option<SkillId>,
    remaining_ms: u64,
    duration_ms: Option<u64>,
    basis: CooldownBasis,
    ready: Option<bool>,
}

impl CooldownSnapshot {
    pub(crate) fn new(
        group_id: CooldownGroupId,
        skill_id: Option<SkillId>,
        remaining_ms: u64,
        duration_ms: Option<u64>,
        basis: CooldownBasis,
    ) -> Self {
        let ready = (basis == CooldownBasis::ServerRemaining && skill_id.is_some())
            .then_some(remaining_ms == 0);

        Self {
            group_id,
            skill_id,
            remaining_ms,
            duration_ms,
            basis,
            ready,
        }
    }

    pub(crate) fn group_id(&self) -> CooldownGroupId {
        self.group_id
    }

    pub(crate) fn skill_id(&self) -> Option<SkillId> {
        self.skill_id
    }

    pub(crate) fn remaining_ms(&self) -> u64 {
        self.remaining_ms
    }

    pub(crate) fn duration_ms(&self) -> Option<u64> {
        self.duration_ms
    }

    pub(crate) fn basis(&self) -> CooldownBasis {
        self.basis
    }

    pub(crate) fn ready(&self) -> Option<bool> {
        self.ready
    }
}
