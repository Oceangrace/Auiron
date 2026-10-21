use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EffectDisplay {
    icon: Option<String>,
    icon_visible: bool,
    remaining_time_visible: bool,
}

impl EffectDisplay {
    pub(crate) fn new(
        icon: Option<String>,
        icon_visible: bool,
        remaining_time_visible: bool,
    ) -> Self {
        Self {
            icon,
            icon_visible,
            remaining_time_visible,
        }
    }

    pub(crate) fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub(crate) fn icon_visible(&self) -> bool {
        self.icon_visible
    }

    pub(crate) fn remaining_time_visible(&self) -> bool {
        self.remaining_time_visible
    }
}
