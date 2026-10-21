use crate::modules::interfaces::rules::ComparisonOperator;
use crate::modules::interfaces::rules::ConditionField;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ConditionCheck {
    #[serde(default)]
    pub(in crate::modules::interfaces) path: Option<String>,
    #[serde(default)]
    pub(in crate::modules::interfaces) text_value: Option<String>,
    pub(in crate::modules::interfaces) field: ConditionField,
    pub(in crate::modules::interfaces) operator: ComparisonOperator,
    pub(in crate::modules::interfaces) value: f64,
}
