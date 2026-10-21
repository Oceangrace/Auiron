use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AnchorMode {
    #[default]
    Start,
    Center,
    End,
    Stretch,
}
