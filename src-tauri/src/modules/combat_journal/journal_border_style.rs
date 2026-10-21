use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
pub(super) enum JournalBorderStyle {
    #[default]
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "dotted")]
    Dotted,
}
