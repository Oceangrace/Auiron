use crate::core::game::CooldownBasis;
use crate::core::game::CooldownGroupId;
use crate::core::game::CooldownSnapshot;
use crate::core::game::SkillId;

pub(super) struct CooldownState {
    group_id: CooldownGroupId,
    skill_id: Option<SkillId>,
    at_us: u64,
    remaining_ms: u64,
    duration_ms: Option<u64>,
    basis: CooldownBasis,
}

impl CooldownState {
    pub(super) fn estimated(
        group_id: CooldownGroupId,
        skill_id: SkillId,
        at_us: u64,
        duration_ms: u32,
    ) -> Self {
        Self {
            group_id,
            skill_id: Some(skill_id),
            at_us,
            remaining_ms: u64::from(duration_ms),
            duration_ms: Some(u64::from(duration_ms)),
            basis: CooldownBasis::BaseDuration,
        }
    }

    pub(super) fn observed(group_id: CooldownGroupId, at_us: u64, remaining_ms: u32) -> Self {
        Self {
            group_id,
            skill_id: None,
            at_us,
            remaining_ms: u64::from(remaining_ms),
            duration_ms: None,
            basis: CooldownBasis::ServerRemaining,
        }
    }

    pub(super) fn update(&mut self, at_us: u64, remaining_ms: u32) {
        self.at_us = at_us;
        self.remaining_ms = u64::from(remaining_ms);
        self.basis = CooldownBasis::ServerRemaining;
    }

    pub(super) fn at_us(&self) -> u64 {
        self.at_us
    }

    pub(super) fn snapshot(&self, now_us: u64) -> CooldownSnapshot {
        let remaining_ms = self
            .remaining_ms
            .saturating_sub(now_us.saturating_sub(self.at_us) / 1000);

        CooldownSnapshot::new(
            self.group_id,
            self.skill_id,
            remaining_ms,
            self.duration_ms,
            self.basis,
        )
    }
}
