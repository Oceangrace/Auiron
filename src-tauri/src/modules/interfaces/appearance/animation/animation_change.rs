use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum AnimationChange {
    #[default]
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "increase")]
    Increase,
    #[serde(rename = "decrease")]
    Decrease,
}
