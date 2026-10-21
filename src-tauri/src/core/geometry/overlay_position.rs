use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct OverlayPosition {
    pub(crate) x: f64,
    pub(crate) y: f64,
}
