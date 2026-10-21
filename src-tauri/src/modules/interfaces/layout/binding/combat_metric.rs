use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum CombatMetric {
    #[serde(rename = "additionalHitEvents")]
    AdditionalHitEvents,
    #[serde(rename = "receivedHits")]
    ReceivedHits,
    #[serde(rename = "regenerationHealing")]
    RegenerationHealing,
    #[serde(rename = "firstEventAt")]
    FirstEventAt,
    #[serde(rename = "lastEventAt")]
    LastEventAt,
    #[serde(rename = "damage")]
    Damage,
    #[serde(rename = "directDamage")]
    DirectDamage,
    #[serde(rename = "periodicDamage")]
    PeriodicDamage,
    #[serde(rename = "healing")]
    Healing,
    #[serde(rename = "periodicHealing")]
    PeriodicHealing,
    #[serde(rename = "damageTaken")]
    DamageTaken,
    #[serde(rename = "healingTaken")]
    HealingTaken,
    #[serde(rename = "hits")]
    Hits,
    #[serde(rename = "directHits")]
    DirectHits,
    #[serde(rename = "periodicHits")]
    PeriodicHits,
    #[serde(rename = "healHits")]
    HealHits,
    #[serde(rename = "criticalHits")]
    CriticalHits,
    #[serde(rename = "backHits")]
    BackHits,
    #[serde(rename = "frontalHits")]
    FrontalHits,
    #[serde(rename = "perfectHits")]
    PerfectHits,
    #[serde(rename = "doubleHits")]
    DoubleHits,
    #[serde(rename = "parryHits")]
    ParryHits,
    #[serde(rename = "shieldBlockHits")]
    ShieldBlockHits,
    #[serde(rename = "ironWallHits")]
    IronWallHits,
    #[serde(rename = "regenerationHits")]
    RegenerationHits,
    #[serde(rename = "perfectBlockHits")]
    PerfectBlockHits,
    #[serde(rename = "misses")]
    Misses,
    #[serde(rename = "resists")]
    Resists,
    #[serde(rename = "additionalHits")]
    AdditionalHits,
    #[serde(rename = "additionalDamage")]
    AdditionalDamage,
    #[serde(rename = "deaths")]
    Deaths,
    #[serde(rename = "dps")]
    Dps,
    #[serde(rename = "hps")]
    Hps,
    #[serde(rename = "criticalPercent")]
    CriticalPercent,
    #[serde(rename = "backPercent")]
    BackPercent,
    #[serde(rename = "frontalPercent")]
    FrontalPercent,
    #[serde(rename = "perfectPercent")]
    PerfectPercent,
    #[serde(rename = "doublePercent")]
    DoublePercent,
    #[serde(rename = "parryPercent")]
    ParryPercent,
    #[serde(rename = "shieldBlockPercent")]
    ShieldBlockPercent,
    #[serde(rename = "missPercent")]
    MissPercent,
    #[serde(rename = "resistPercent")]
    ResistPercent,
    #[serde(rename = "averageHit")]
    AverageHit,
    #[serde(rename = "minHit")]
    MinHit,
    #[serde(rename = "maxHit")]
    MaxHit,
    #[serde(rename = "duration")]
    Duration,
}
