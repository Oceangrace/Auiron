use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum GlowMode {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "proc")]
    Proc,
    #[serde(rename = "pixel")]
    Pixel,
}
