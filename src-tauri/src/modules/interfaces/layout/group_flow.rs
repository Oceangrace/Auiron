use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum GroupFlow {
    #[default]
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "timeline")]
    Timeline,
}
