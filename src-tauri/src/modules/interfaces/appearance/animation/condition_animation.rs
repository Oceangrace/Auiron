use super::AnimationChange;
use super::AnimationWhen;
use super::WidgetAnimation;
use crate::modules::interfaces::appearance::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ConditionAnimation {
    pub(in crate::modules::interfaces) preset: WidgetAnimation,
    pub(in crate::modules::interfaces) when: AnimationWhen,
    pub(in crate::modules::interfaces) color: HexColor,
    #[schemars(range(min = 100, max = 10000))]
    pub(in crate::modules::interfaces) period_ms: f64,
    #[schemars(range(min = 0, max = 100))]
    pub(in crate::modules::interfaces) intensity: f64,
    #[schemars(range(min = 0, max = 3600))]
    pub(in crate::modules::interfaces) threshold_seconds: f64,
    #[schemars(range(min = 100, max = 60000))]
    pub(in crate::modules::interfaces) duration_ms: f64,
    #[schemars(length(max = 128))]
    pub(in crate::modules::interfaces) path: String,
    pub(in crate::modules::interfaces) change: AnimationChange,
}
