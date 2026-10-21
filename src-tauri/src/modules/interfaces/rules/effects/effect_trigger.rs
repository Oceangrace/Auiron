use super::EffectFilter;
use super::EffectOrigin;
use super::EffectPresence;
use super::TriggerUnit;
use crate::integrations::aion2::GameId;
use crate::modules::interfaces::layout::Direction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EffectTrigger {
    pub(in crate::modules::interfaces) unit: TriggerUnit,
    #[schemars(length(max = 64))]
    pub(in crate::modules::interfaces) member: String,
    pub(in crate::modules::interfaces) filter: EffectFilter,
    #[schemars(length(max = 32))]
    pub(in crate::modules::interfaces) effects: Vec<GameId>,
    pub(in crate::modules::interfaces) presence: EffectPresence,
    pub(in crate::modules::interfaces) origin: EffectOrigin,
    #[schemars(range(min = 1, max = 999))]
    pub(in crate::modules::interfaces) min_stacks: f64,
    #[schemars(range(min = 0, max = 3600))]
    pub(in crate::modules::interfaces) remaining_below_seconds: f64,
    pub(in crate::modules::interfaces) clones: bool,
    pub(in crate::modules::interfaces) clone_direction: Direction,
}
