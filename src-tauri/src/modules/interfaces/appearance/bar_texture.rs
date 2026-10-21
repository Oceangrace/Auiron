use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BarTexture {
    #[default]
    Solid,
    Grid,
    Diagonal,
}
