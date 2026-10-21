use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum OverlayAnchor {
    TopLeft,
    #[default]
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Free,
}
