use super::ConditionMode;
use super::TriggerRule;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct TriggerGroup {
    pub(in crate::modules::interfaces) mode: ConditionMode,
    pub(in crate::modules::interfaces) data_rule_id: Option<String>,
    #[schemars(length(min = 1, max = 8))]
    pub(in crate::modules::interfaces) rules: Vec<TriggerRule>,
}
