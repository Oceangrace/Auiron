use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ObservationKind {
    SkillUsed,
    SkillState,
    EffectPresent,
    Cooldown,
    SkillReady,
    ComboAvailable,
}
