use crate::core::game::CombatCounter;
use crate::core::game::CombatStatistics;
use crate::core::game::EntityId;
use crate::core::game::GameSnapshot;
use crate::modules::interfaces::layout::CombatMetric;
use serde_json::{Value, json};
use std::collections::HashMap;

pub(super) struct CombatProjection;

impl CombatProjection {
    pub(super) fn actor(game: &GameSnapshot, id: EntityId) -> Option<CombatStatistics> {
        let mut result = CombatStatistics::default();

        let mut observed = false;

        for record in game.statistics() {
            let statistics = record.statistics();

            if game.root_actor(record.source_id()) == Some(id) {
                if record.source_id() == id {
                    result.merge(statistics);
                } else {
                    result.merge_owned_actions(statistics);
                }

                observed = true;
            }

            if record.recipient_id() == id {
                result.increment(
                    CombatCounter::DamageTaken,
                    statistics.count(CombatCounter::Damage),
                );
                result.increment(
                    CombatCounter::HealingTaken,
                    statistics.count(CombatCounter::Healing),
                );
                result.increment(
                    CombatCounter::ReceivedHits,
                    statistics.count(CombatCounter::Hits),
                );
                result.increment(
                    CombatCounter::RegenerationHealing,
                    statistics.count(CombatCounter::RestoredHp),
                );

                if let Some(at) = statistics.first_at_us() {
                    result.touch(at);
                }

                if let Some(at) = statistics.last_at_us() {
                    result.touch(at);
                }

                if !statistics.complete() {
                    result.incomplete();
                }

                observed = true;
            }
        }

        observed.then_some(result)
    }

    pub(super) fn fields(
        fields: &mut HashMap<String, Value>,
        prefix: &str,
        statistics: &CombatStatistics,
        duration_ms: u64,
    ) {
        fields.insert(format!("{prefix}.complete"), json!(statistics.complete()));
        fields.insert(
            format!("{prefix}.observedDamage"),
            json!(statistics.count(CombatCounter::Damage)),
        );
        fields.insert(
            format!("{prefix}.unknownHits"),
            json!(statistics.count(CombatCounter::UnknownHits)),
        );

        let metrics = [
            ("additionalHitEvents", CombatMetric::AdditionalHitEvents),
            ("receivedHits", CombatMetric::ReceivedHits),
            ("regenerationHealing", CombatMetric::RegenerationHealing),
            ("damage", CombatMetric::Damage),
            ("directDamage", CombatMetric::DirectDamage),
            ("periodicDamage", CombatMetric::PeriodicDamage),
            ("healing", CombatMetric::Healing),
            ("periodicHealing", CombatMetric::PeriodicHealing),
            ("damageTaken", CombatMetric::DamageTaken),
            ("healingTaken", CombatMetric::HealingTaken),
            ("hits", CombatMetric::Hits),
            ("directHits", CombatMetric::DirectHits),
            ("periodicHits", CombatMetric::PeriodicHits),
            ("healHits", CombatMetric::HealHits),
            ("criticalHits", CombatMetric::CriticalHits),
            ("backHits", CombatMetric::BackHits),
            ("frontalHits", CombatMetric::FrontalHits),
            ("perfectHits", CombatMetric::PerfectHits),
            ("doubleHits", CombatMetric::DoubleHits),
            ("parryHits", CombatMetric::ParryHits),
            ("shieldBlockHits", CombatMetric::ShieldBlockHits),
            ("ironWallHits", CombatMetric::IronWallHits),
            ("regenerationHits", CombatMetric::RegenerationHits),
            ("perfectBlockHits", CombatMetric::PerfectBlockHits),
            ("misses", CombatMetric::Misses),
            ("resists", CombatMetric::Resists),
            ("additionalHits", CombatMetric::AdditionalHits),
            ("additionalDamage", CombatMetric::AdditionalDamage),
            ("deaths", CombatMetric::Deaths),
            ("firstEventAt", CombatMetric::FirstEventAt),
            ("lastEventAt", CombatMetric::LastEventAt),
            ("dps", CombatMetric::Dps),
            ("hps", CombatMetric::Hps),
            ("duration", CombatMetric::Duration),
            ("averageHit", CombatMetric::AverageHit),
            ("minHit", CombatMetric::MinHit),
            ("maxHit", CombatMetric::MaxHit),
            ("criticalPercent", CombatMetric::CriticalPercent),
            ("backPercent", CombatMetric::BackPercent),
            ("frontalPercent", CombatMetric::FrontalPercent),
            ("perfectPercent", CombatMetric::PerfectPercent),
            ("doublePercent", CombatMetric::DoublePercent),
            ("parryPercent", CombatMetric::ParryPercent),
            ("shieldBlockPercent", CombatMetric::ShieldBlockPercent),
            ("missPercent", CombatMetric::MissPercent),
            ("resistPercent", CombatMetric::ResistPercent),
        ];

        for (path, metric) in metrics {
            if let Some(value) = Self::value(statistics, metric, duration_ms) {
                fields.insert(format!("{prefix}.{path}"), json!(value));
            }
        }
    }

    pub(super) fn value(
        statistics: &CombatStatistics,
        metric: CombatMetric,
        duration_ms: u64,
    ) -> Option<f64> {
        let count = |counter| statistics.count(counter) as f64;

        let ratio =
            |numerator, denominator| (denominator > 0.0).then_some(numerator / denominator * 100.0);

        let seconds = (duration_ms as f64 / 1000.0).max(1.0);

        let complete = statistics.complete();

        Some(match metric {
            CombatMetric::AdditionalHitEvents => count(CombatCounter::AdditionalHitEvents),
            CombatMetric::ReceivedHits => count(CombatCounter::ReceivedHits),
            CombatMetric::RegenerationHealing => count(CombatCounter::RegenerationHealing),
            CombatMetric::Damage => {
                if !complete {
                    return None;
                }

                count(CombatCounter::Damage)
            }

            CombatMetric::DirectDamage => count(CombatCounter::DirectDamage),
            CombatMetric::PeriodicDamage => count(CombatCounter::PeriodicDamage),
            CombatMetric::Healing => count(CombatCounter::Healing),
            CombatMetric::PeriodicHealing => count(CombatCounter::PeriodicHealing),
            CombatMetric::DamageTaken => {
                if !complete {
                    return None;
                }

                count(CombatCounter::DamageTaken)
            }

            CombatMetric::HealingTaken => count(CombatCounter::HealingTaken),
            CombatMetric::Hits => count(CombatCounter::Hits),
            CombatMetric::DirectHits => count(CombatCounter::DirectHits),
            CombatMetric::PeriodicHits => count(CombatCounter::PeriodicHits),
            CombatMetric::HealHits => count(CombatCounter::HealHits),
            CombatMetric::CriticalHits => count(CombatCounter::CriticalHits),
            CombatMetric::BackHits => count(CombatCounter::BackHits),
            CombatMetric::FrontalHits => count(CombatCounter::FrontalHits),
            CombatMetric::PerfectHits => count(CombatCounter::PerfectHits),
            CombatMetric::DoubleHits => count(CombatCounter::DoubleHits),
            CombatMetric::ParryHits => count(CombatCounter::ParryHits),
            CombatMetric::ShieldBlockHits => count(CombatCounter::ShieldBlockHits),
            CombatMetric::IronWallHits => count(CombatCounter::IronWallHits),
            CombatMetric::RegenerationHits => count(CombatCounter::RegenerationHits),
            CombatMetric::PerfectBlockHits => count(CombatCounter::PerfectBlockHits),
            CombatMetric::Misses => count(CombatCounter::Misses),
            CombatMetric::Resists => count(CombatCounter::Resists),
            CombatMetric::AdditionalHits => count(CombatCounter::AdditionalHits),
            CombatMetric::AdditionalDamage => count(CombatCounter::AdditionalDamage),
            CombatMetric::Deaths => count(CombatCounter::Deaths),
            CombatMetric::FirstEventAt => statistics.first_at_us()? as f64 / 1000.0,
            CombatMetric::LastEventAt => statistics.last_at_us()? as f64 / 1000.0,
            CombatMetric::Duration => duration_ms as f64,
            CombatMetric::Dps => {
                if !complete {
                    return None;
                }

                count(CombatCounter::Damage) / seconds
            }

            CombatMetric::Hps => count(CombatCounter::Healing) / seconds,
            CombatMetric::AverageHit => {
                if !complete || statistics.count(CombatCounter::Hits) == 0 {
                    return None;
                }

                count(CombatCounter::Damage) / count(CombatCounter::Hits)
            }

            CombatMetric::MinHit => statistics.minimum()? as f64,
            CombatMetric::MaxHit => statistics.maximum()? as f64,
            CombatMetric::CriticalPercent => ratio(
                count(CombatCounter::CriticalHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::BackPercent => ratio(
                count(CombatCounter::BackHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::FrontalPercent => ratio(
                count(CombatCounter::FrontalHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::PerfectPercent => ratio(
                count(CombatCounter::PerfectHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::DoublePercent => ratio(
                count(CombatCounter::DoubleHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::ParryPercent => ratio(
                count(CombatCounter::ParryHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::ShieldBlockPercent => ratio(
                count(CombatCounter::ShieldBlockHits),
                count(CombatCounter::DirectHits),
            )?,
            CombatMetric::MissPercent => ratio(
                count(CombatCounter::Misses),
                count(CombatCounter::DirectHits)
                    + count(CombatCounter::Misses)
                    + count(CombatCounter::Resists),
            )?,
            CombatMetric::ResistPercent => ratio(
                count(CombatCounter::Resists),
                count(CombatCounter::DirectHits)
                    + count(CombatCounter::Misses)
                    + count(CombatCounter::Resists),
            )?,
        })
    }

    pub(super) fn duration(game: &GameSnapshot) -> u64 {
        game.combat()
            .and_then(|combat| combat.elapsed_ms())
            .unwrap_or(0)
    }
}
