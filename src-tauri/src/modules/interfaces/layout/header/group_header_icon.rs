use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum GroupHeaderIcon {
    Edit,
    Settings,
    Chevron,
    Chart,
    Close,
    Panels,
}
