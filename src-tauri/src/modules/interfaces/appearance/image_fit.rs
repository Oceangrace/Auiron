use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ImageFit {
    #[default]
    Cover,
    Contain,
    Stretch,
}
