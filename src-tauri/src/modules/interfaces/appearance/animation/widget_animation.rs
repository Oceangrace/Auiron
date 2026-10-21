use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum WidgetAnimation {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pulse")]
    Pulse,
    #[serde(rename = "blink")]
    Blink,
    #[serde(rename = "bounce")]
    Bounce,
    #[serde(rename = "glow")]
    Glow,
    #[serde(rename = "border")]
    Border,
    #[serde(rename = "tint")]
    Tint,
}
