use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize, schemars::JsonSchema)]
pub(crate) enum GroupGrow {
    #[default]
    #[serde(rename = "forward")]
    Forward,
    #[serde(rename = "backward")]
    Backward,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "end-start")]
    EndStart,
    #[serde(rename = "start-end")]
    StartEnd,
}

impl GroupGrow {
    pub(in crate::modules::interfaces) fn offset(
        self,
        horizontal: bool,
        available: f64,
        total: f64,
    ) -> f64 {
        let end = match self {
            Self::Forward => false,
            Self::Backward => true,
            Self::Center => return (available - total) / 2.0,
            Self::EndStart => horizontal,
            Self::StartEnd => !horizontal,
        };

        if end { available - total } else { 0.0 }
    }
}
