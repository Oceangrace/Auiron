use super::cooldown_state::CooldownState;
use crate::core::game::ComboAvailability;
use crate::core::game::CooldownGroupId;
use crate::core::game::CooldownSnapshot;
use crate::core::game::SkillId;
use crate::integrations::aion2::catalog::RuntimeCatalog;
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct SkillTracker {
    cooldowns: BTreeMap<CooldownGroupId, CooldownState>,
    combos: BTreeMap<SkillId, ComboAvailability>,
}

impl SkillTracker {
    const MAX_ENTRIES: usize = 4096;

    pub(super) fn used(
        &mut self,
        skill_id: SkillId,
        at_us: u64,
        confirmed: bool,
        catalog: &RuntimeCatalog,
    ) -> Result<(), String> {
        if self.combos.contains_key(&skill_id) {
            self.combo(skill_id, false, at_us)?;
        }

        let Some(skill) = catalog.skill(skill_id.get()) else {
            return Ok(());
        };

        let Some(group) = skill.cooldown_group() else {
            return Ok(());
        };

        let group_id = CooldownGroupId::new(group)?;

        if confirmed
            && self
                .cooldowns
                .get(&group_id)
                .is_some_and(|state| at_us.saturating_sub(state.at_us()) <= 100_000)
        {
            return Ok(());
        }

        if self.cooldowns.len() >= Self::MAX_ENTRIES && !self.cooldowns.contains_key(&group_id) {
            return Err("Observed cooldown limit".into());
        }

        self.cooldowns.insert(
            group_id,
            CooldownState::estimated(group_id, skill_id, at_us, skill.cooldown_ms()),
        );

        Ok(())
    }

    pub(super) fn remaining(
        &mut self,
        group: CooldownGroupId,
        remaining_ms: u32,
        at_us: u64,
    ) -> Result<(), String> {
        if let Some(state) = self.cooldowns.get_mut(&group) {
            state.update(at_us, remaining_ms);

            return Ok(());
        }

        if self.cooldowns.len() >= Self::MAX_ENTRIES {
            return Err("Observed cooldown limit".into());
        }

        self.cooldowns
            .insert(group, CooldownState::observed(group, at_us, remaining_ms));

        Ok(())
    }

    pub(super) fn combo(
        &mut self,
        skill: SkillId,
        available: bool,
        at_us: u64,
    ) -> Result<(), String> {
        if self.combos.len() >= Self::MAX_ENTRIES && !self.combos.contains_key(&skill) {
            return Err("Observed combo limit".into());
        }

        self.combos
            .insert(skill, ComboAvailability::new(skill, available, at_us));

        Ok(())
    }

    pub(super) fn cooldowns(&self, now_us: u64) -> Vec<CooldownSnapshot> {
        self.cooldowns
            .values()
            .map(|state| state.snapshot(now_us))
            .collect()
    }

    pub(super) fn combos(&self, now_us: u64, catalog: &RuntimeCatalog) -> Vec<ComboAvailability> {
        self.combos
            .values()
            .map(|combo| {
                let window = catalog
                    .skill(combo.skill_id().get())
                    .map(|skill| skill.chain_window_ms());
                let expired = window.is_some_and(|ms| {
                    ms > 0 && now_us.saturating_sub(combo.at_us()) >= u64::from(ms) * 1000
                });

                ComboAvailability::new(
                    combo.skill_id(),
                    combo.available() && !expired,
                    combo.at_us(),
                )
            })
            .collect()
    }
}
