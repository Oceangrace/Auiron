use serde::Serialize;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum TriggerState {
    Active,
    Inactive,
    Unknown,
    Unsupported,
}

impl TriggerState {
    pub(super) fn active(self) -> Option<bool> {
        match self {
            Self::Active => Some(true),
            Self::Inactive => Some(false),
            _ => None,
        }
    }

    pub(super) fn from_active(active: Option<bool>, unsupported: bool) -> Self {
        match active {
            Some(true) => Self::Active,
            Some(false) => Self::Inactive,
            None if unsupported => Self::Unsupported,
            None => Self::Unknown,
        }
    }
}
