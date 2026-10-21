use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum LoadState {
    #[default]
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
}
