use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum GroupSort {
    #[default]
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "remaining-asc")]
    RemainingAsc,
    #[serde(rename = "remaining-desc")]
    RemainingDesc,
}
