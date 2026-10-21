use crate::core::game::CombatCounter;
use crate::core::game::CombatStatistics;
use crate::core::game::GameEvent;
use crate::core::game::HitModifier;
use crate::core::game::HitOutcome;

pub(super) struct CombatAccumulator;

impl CombatAccumulator {
    pub(super) fn observe(statistics: &mut CombatStatistics, event: &GameEvent, at_us: u64) {
        statistics.touch(at_us);

        match event {
            GameEvent::Damage { value, hit, .. } => {
                statistics.increment(CombatCounter::Damage, u64::from(*value));
                statistics.increment(
                    CombatCounter::RestoredHp,
                    u64::from(hit.restored_hp().unwrap_or(0)),
                );
                statistics.increment(CombatCounter::Hits, 1);
                statistics.range(u64::from(value.saturating_sub(hit.additional_damage())));

                let (damage, hits) = if hit.periodic() {
                    (CombatCounter::PeriodicDamage, CombatCounter::PeriodicHits)
                } else {
                    (CombatCounter::DirectDamage, CombatCounter::DirectHits)
                };

                statistics.increment(damage, u64::from(*value));
                statistics.increment(hits, 1);

                if hit.outcome() == Some(HitOutcome::Critical) {
                    statistics.increment(CombatCounter::CriticalHits, 1);
                }

                statistics.increment(
                    CombatCounter::AdditionalHits,
                    u64::from(hit.additional_hits()),
                );
                statistics.increment(
                    CombatCounter::AdditionalDamage,
                    u64::from(hit.additional_damage()),
                );

                if hit.additional_hits() > 0 {
                    statistics.increment(CombatCounter::AdditionalHitEvents, 1);
                }

                for modifier in hit.modifiers() {
                    statistics.increment(Self::modifier(*modifier), 1);
                }
            }

            GameEvent::Healing { value, hit, .. } => {
                statistics.increment(CombatCounter::Healing, u64::from(*value));
                statistics.increment(CombatCounter::HealHits, 1);

                if hit.as_ref().is_some_and(|hit| hit.periodic()) {
                    statistics.increment(CombatCounter::PeriodicHealing, u64::from(*value));
                }
            }

            GameEvent::Miss { .. } => statistics.increment(CombatCounter::Misses, 1),
            GameEvent::Resist { .. } => statistics.increment(CombatCounter::Resists, 1),
            GameEvent::DamageUnknown { .. } => {
                statistics.increment(CombatCounter::UnknownHits, 1);
                statistics.incomplete();
            }

            _ => {}
        }
    }

    fn modifier(modifier: HitModifier) -> CombatCounter {
        match modifier {
            HitModifier::Back => CombatCounter::BackHits,
            HitModifier::Frontal => CombatCounter::FrontalHits,
            HitModifier::Perfect => CombatCounter::PerfectHits,
            HitModifier::Double => CombatCounter::DoubleHits,
            HitModifier::Parry => CombatCounter::ParryHits,
            HitModifier::ShieldBlock => CombatCounter::ShieldBlockHits,
            HitModifier::IronWall => CombatCounter::IronWallHits,
            HitModifier::Regeneration => CombatCounter::RegenerationHits,
            HitModifier::PerfectBlock => CombatCounter::PerfectBlockHits,
        }
    }
}
