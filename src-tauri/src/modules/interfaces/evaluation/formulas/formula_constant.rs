use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(untagged)]
pub(crate) enum FormulaConstant {
    Boolean(bool),
    Number(serde_json::Number),
}
