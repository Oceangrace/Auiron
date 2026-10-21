use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum ScreenPreset {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "720p")]
    Hd,
    #[serde(rename = "1080p")]
    FullHd,
    #[serde(rename = "1440p")]
    Qhd,
}
