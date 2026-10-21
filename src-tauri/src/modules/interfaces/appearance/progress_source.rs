use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum ProgressSource {
    #[default]
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "trigger")]
    Trigger,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "field")]
    Field,
}
