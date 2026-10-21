use crate::modules::interfaces::rules::ConditionChange;
use crate::modules::interfaces::rules::ConditionCheck;
use crate::modules::interfaces::rules::ConditionMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct WidgetCondition {
    pub(in crate::modules::interfaces) id: String,
    pub(in crate::modules::interfaces) enabled: bool,
    pub(in crate::modules::interfaces) mode: ConditionMode,
    pub(in crate::modules::interfaces) checks: Vec<ConditionCheck>,
    pub(in crate::modules::interfaces) changes: Vec<ConditionChange>,
}
