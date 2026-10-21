use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize, schemars::JsonSchema)]
pub(super) enum JournalHeaderButton {
    #[serde(rename = "metric")]
    Metric,
    #[serde(rename = "segment")]
    Segment,
    #[serde(rename = "new-window")]
    NewWindow,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "new-run")]
    NewRun,
    #[serde(rename = "settings")]
    Settings,
    #[serde(rename = "close")]
    Close,
}

impl JournalHeaderButton {
    pub(super) fn defaults() -> Vec<Self> {
        vec![
            Self::Metric,
            Self::Segment,
            Self::NewWindow,
            Self::Preview,
            Self::Settings,
            Self::Close,
        ]
    }
}
