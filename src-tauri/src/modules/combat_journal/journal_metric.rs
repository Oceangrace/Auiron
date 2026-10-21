use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(super) enum JournalMetric {
    Damage,
    Dps,
    Healing,
    Hps,
    DamageTaken,
    HealingTaken,
    Hits,
    Critical,
    Deaths,
    Misses,
    Resists,
}
