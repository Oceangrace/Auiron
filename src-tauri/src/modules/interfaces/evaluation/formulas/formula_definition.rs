use super::FormulaExpression;
use super::FormulaValueKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct FormulaDefinition {
    #[schemars(length(min = 1, max = 100))]
    pub(in crate::modules::interfaces) id: String,
    #[schemars(length(min = 1, max = 128))]
    pub(in crate::modules::interfaces) name: String,
    pub(in crate::modules::interfaces) kind: FormulaValueKind,
    pub(in crate::modules::interfaces) expression: FormulaExpression,
}
