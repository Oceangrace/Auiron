use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum ConditionField {
    #[default]
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "remaining")]
    Remaining,
    #[serde(rename = "stacks")]
    Stacks,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "field")]
    Custom,
}
