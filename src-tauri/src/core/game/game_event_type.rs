use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum GameEventType {
    CastStarted,
    HitLanded,
    ServerBattleChanged,
    CombatStarted,
    CombatEnded,
    EncounterStarted,
    EncounterEnded,
    Healing,
    Miss,
    Resist,
    Death,
    Despawn,
    MapChanged,
    Damage,
    HealthChanged,
    EffectApplied,
    EffectUpdated,
    EffectRemoved,
    SkillUsed,
    ComboAvailable,
    ComboUnavailable,
}
