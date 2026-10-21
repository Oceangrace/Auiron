use crate::core::game::EffectDisplay;
use crate::core::game::EffectDisposition;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EffectMetadata {
    disposition: EffectDisposition,
    stack_limit: NonZeroU16,
    icon: Option<String>,
    icon_visible: bool,
    remaining_time_visible: bool,
    #[serde(default)]
    enables_shock_removal: bool,
    #[serde(default)]
    healing_rows: Vec<u8>,
}

impl EffectMetadata {
    pub(crate) fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    pub(crate) fn healing_rows(&self) -> &[u8] {
        &self.healing_rows
    }

    pub(crate) fn enables_shock_removal(&self) -> bool {
        self.enables_shock_removal
    }

    pub(crate) fn disposition(&self) -> EffectDisposition {
        self.disposition
    }

    pub(crate) fn stack_limit(&self) -> usize {
        usize::from(self.stack_limit.get())
    }

    pub(crate) fn display(&self) -> EffectDisplay {
        EffectDisplay::new(
            self.icon.clone(),
            self.icon_visible,
            self.remaining_time_visible,
        )
    }
}
