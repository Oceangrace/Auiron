use super::ValueOperator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ValueTrigger {
    #[schemars(length(min = 1, max = 128))]
    pub(in crate::modules::interfaces) path: String,
    pub(in crate::modules::interfaces) operator: ValueOperator,
    #[schemars(length(max = 128))]
    pub(in crate::modules::interfaces) value: String,
}
