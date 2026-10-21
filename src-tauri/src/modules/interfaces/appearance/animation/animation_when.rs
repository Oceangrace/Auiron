use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum AnimationWhen {
    #[default]
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "effect-active")]
    EffectActive,
    #[serde(rename = "remaining")]
    Remaining,
    #[serde(rename = "elapsed")]
    Elapsed,
    #[serde(rename = "on-show")]
    OnShow,
    #[serde(rename = "value-changed")]
    ValueChanged,
}
