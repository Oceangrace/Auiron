use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ClassColorTarget {
    #[default]
    Fill,
    Background,
    Text,
    Border,
}
