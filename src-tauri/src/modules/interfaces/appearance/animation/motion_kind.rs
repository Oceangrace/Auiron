use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum MotionKind {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "fade")]
    Fade,
    #[serde(rename = "grow")]
    Grow,
    #[serde(rename = "slide-up")]
    SlideUp,
    #[serde(rename = "slide-down")]
    SlideDown,
    #[serde(rename = "slide-left")]
    SlideLeft,
    #[serde(rename = "slide-right")]
    SlideRight,
}
